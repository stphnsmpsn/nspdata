use crate::{BidgelyError, BIDGELY_BASE_URL};

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SessionResponse {
    pub request_id: String,
    pub payload: SessionPayload,
    pub error: Option<String>,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct SessionPayload {
    pub pilot_id: u64,
    pub client_id: String,
    pub token_details: TokenDetails,
    pub user_profile_details: UserProfileDetails,
    pub user_type_details: UserTypeDetails,
    pub premises_details: PremisesDetails,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct TokenDetails {
    pub access_token: String,
    pub expiry_time_in_millis: u64,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserProfileDetails {
    pub user_id: String,
    pub partner_user_id: String,
    pub meter_id: Option<String>,
    pub first_name: String,
    pub last_name: String,
    pub fuel_type: String, // todo: type this as enum FuelType
    pub email: String,
    pub utility_tags: UtilityTags,
    pub home_accounts: HomeAccounts,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "snake_case"))]
pub struct UtilityTags {
    pub account_number: String,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct HomeAccounts {
    pub address: String,
    pub has_solar: bool,
    pub postal_code: String,
    pub rate: RatePlanInfo,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct RatePlanInfo {
    pub rate_plan_id: String,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct UserTypeDetails {
    pub user_segment: String, // todo: type this as enum UserSegment
    pub measurement_to_user_type_mappings: Vec<MeasurementToUserType>,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct MeasurementToUserType {
    pub measurement_type: String,
    pub user_type: String,
    pub max_contract_end: u32,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PremisesDetails {
    pub partner_user_id: String,
    pub premises: Vec<Premise>,
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct Premise {
    pub uuid: String,
    pub premise_id: Option<String>,
    pub address: PremiseAddress,
    pub supported_measurement_types: Vec<String>, // toto: type this as enum MeasurementType
    pub dashboard_last_visited: u32,
    pub status: String, // todo: type this as enum PremiseStatus
}

#[derive(Debug, Default, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all(deserialize = "camelCase"))]
pub struct PremiseAddress {
    pub address_line_1: String,
    pub address_line_2: Option<String>,
    pub city: String,
    pub state: String,
    pub zipcode: String,
    pub context: Option<String>,
}

pub async fn session(session: &str) -> Result<SessionResponse, BidgelyError> {
    Ok(serde_json::from_str(
        &reqwest::get(format!(
            "{BIDGELY_BASE_URL}/web/web-session/{session}?pilotId=40003&clientId=nsp-dashboard"
        ))
        .await?
        .text()
        .await?,
    )?)
}
