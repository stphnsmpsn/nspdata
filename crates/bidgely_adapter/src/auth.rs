use crate::{BidgelyError, BIDGELY_BASE_URL};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserAuthResponse {
    pub request_id: String,
    pub payload: String,
    pub error: Option<String>,
}

pub async fn auth(user_id: &str) -> Result<UserAuthResponse, BidgelyError> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "{BIDGELY_BASE_URL}/user-auth/cipher?user-id={user_id}&pilot-id=40003"
        ))
        .await?
        .text()
        .await?,
    )?)
}
