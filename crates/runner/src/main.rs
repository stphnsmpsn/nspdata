use bidgely_adapter::{SessionResponse, UserAuthResponse};
use clap::Parser;

const BIDGELY_BASE_URL: &'static str = "https://caapi.bidgely.com/v2.0";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    uid: String,
}

#[tokio::main]
async fn main() {
    let args: Args = Args::parse();
    let user_id = args.uid.as_str();

    let user_auth_response: UserAuthResponse = serde_json::from_str(
        &reqwest::get(format!(
            "{BIDGELY_BASE_URL}/user-auth/cipher?user-id={user_id}&pilot-id=40003"
        ))
        .await
        .unwrap()
        .text()
        .await
        .unwrap(),
    )
    .unwrap();

    let session = user_auth_response.payload;

    let session_response: SessionResponse = serde_json::from_str(
        &reqwest::get(format!(
            "{BIDGELY_BASE_URL}/web/web-session/{session}?pilotId=40003&clientId=nsp-dashboard"
        ))
        .await
        .unwrap()
        .text()
        .await
        .unwrap(),
    )
    .unwrap();

    let token = session_response.payload.token_details.access_token;

    let client = reqwest::Client::new();
    let xml_data = client.get(format!(
        "{BIDGELY_BASE_URL}/dashboard/users/{user_id}/gb-download?start=1660694400&end=1665964800&measurement-type=ELECTRIC"
    ))
        .header(reqwest::header::CONTENT_TYPE, "application/json;charset=UTF-8")
        .header(reqwest::header::AUTHORIZATION, format!("Bearer {token}"))
        .send()
    .await
    .unwrap()
    .text()
    .await
    .unwrap();

    println!("{:?}", xml_data);
}
