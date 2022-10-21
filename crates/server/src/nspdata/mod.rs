pub(crate) mod cache;
pub(crate) mod search;

use crate::Context;
use rocket::State;

#[get("/<user_id>?<start>&<end>")]
pub(crate) async fn handle_search(
    context: &State<Context>,
    user_id: String,
    start: u64,
    end: u64,
) -> String {
    // todo: impl Responder for ServerError to eliminate these unwraps
    serde_json::to_string(&search::handler(context, user_id, start, end).await.unwrap()).unwrap()
}
