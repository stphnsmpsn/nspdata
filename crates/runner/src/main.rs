use crate::args::{Action, Args};
use clap::Parser;

pub mod args;
pub mod convert;
pub mod download;
pub mod error;

const BIDGELY_BASE_URL: &'static str = "https://caapi.bidgely.com/v2.0";

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    let args: Args = Args::parse();

    match args.action {
        Action::Convert => convert::convert(&args)?,
        Action::Download => download::download(BIDGELY_BASE_URL, &args).await?,
    }

    let user_id = "your-user-id";
    let start = 1664593200;
    let end = 1665413999;
    let user_auth_response = bidgely_adapter::auth::auth(BIDGELY_BASE_URL, user_id).await?;
    let session_response =
        bidgely_adapter::session::session(BIDGELY_BASE_URL, user_auth_response.payload.as_str())
            .await?;
    let feed = bidgely_adapter::feed::get_feed(
        BIDGELY_BASE_URL,
        user_id,
        session_response.payload.token_details.access_token.as_str(),
        start,
        end,
    )
    .await?;

    let interval_blocks: Vec<bidgely_adapter::feed::IntervalBlock> = feed
        .entry
        .into_iter()
        .filter_map(|entry| match entry.content.to_inner() {
            bidgely_adapter::feed::ContentType::IntervalBlock(e) => Some(e),
            _ => None,
        })
        .collect();

    let mut days: Vec<u32> = vec![];

    interval_blocks.iter().for_each(|interval_block| {
        let total = interval_block
            .interval_reading
            .iter()
            .fold(0, |acc, x| acc + x.value);
        days.push(total)
    });

    days.iter().for_each(|day| println!("{:?}", day));

    Ok(())
}
