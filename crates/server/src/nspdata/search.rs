use crate::{Context, ServerError};
use bidgely_adapter::feed::{ContentType, IntervalReading};
use bidgely_adapter::session::{SessionResponse, UserProfileDetails};
use rocket::State;
use std::ops::DerefMut;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct SearchResponse {
    user_profile: UserProfileDetails,
    readings: Vec<PrettyIntervalReading>,
}

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub(crate) struct PrettyIntervalReading {
    start: u64,
    duration: u64,
    quality: u32,
    value: u32,
}

impl From<(SessionResponse, Vec<IntervalReading>)> for SearchResponse {
    fn from((session_response, readings): (SessionResponse, Vec<IntervalReading>)) -> Self {
        Self {
            user_profile: session_response.payload.user_profile_details,
            readings: readings
                .into_iter()
                .map(|reading| PrettyIntervalReading {
                    start: reading.time_period.start,
                    duration: reading.time_period.duration,
                    quality: reading.reading_quality.quality,
                    value: reading.value,
                })
                .collect::<Vec<PrettyIntervalReading>>(),
        }
    }
}

pub(crate) async fn handler(
    context: &State<Context>,
    user_id: String,
    start: u64,
    end: u64,
) -> Result<SearchResponse, crate::error::ServerError> {
    let session_response = get_session(context, user_id.clone()).await?;

    let mut feed = bidgely_adapter::feed::get_feed(
        user_id.as_str(),
        session_response.payload.token_details.access_token.as_str(),
        start,
        end,
    )
    .await?;

    // todo: think if there is a nicer way to do this via some sort of filter / retain / map / collect as opposed
    // to creating a new vector and moving elements into it.
    let mut return_data: Vec<IntervalReading> = vec![];

    feed.entries
        .iter_mut()
        .for_each(|entry| match entry.content.deref_mut() {
            ContentType::IntervalBlock(interval_block) => {
                interval_block.interval_reading.retain(|interval_reading| {
                    (interval_reading.time_period.start > start)
                        && ((interval_reading.time_period.start
                            + interval_reading.time_period.duration)
                            <= end)
                });
                return_data.append(&mut interval_block.interval_reading)
            }
            _ => {}
        });

    Ok((session_response, return_data).into())
}

async fn get_session(
    context: &State<Context>,
    user_id: String,
) -> Result<SessionResponse, ServerError> {
    let cache = context.nsp_cache();

    Ok(match cache.find_session(&user_id).await? {
        Some(session) => session,
        None => {
            let user_auth_response = bidgely_adapter::auth::auth(user_id.as_str()).await.unwrap();

            let session_response =
                bidgely_adapter::session::session(user_auth_response.payload.as_str()).await?;

            cache
                .store_session(user_id.clone(), session_response)
                .await?
                .unwrap()
        }
    })
}
