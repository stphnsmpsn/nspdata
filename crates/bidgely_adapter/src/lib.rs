pub mod auth;
pub mod feed;
pub mod session;

pub(crate) const BIDGELY_BASE_URL: &'static str = "https://caapi.bidgely.com/v2.0";

#[derive(thiserror::Error, Debug)]
pub enum BidgelyError {
    #[error("reqwest error")]
    Reqwest(#[from] reqwest::Error),
    #[error("Serde JSON Error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Quick XML De Error")]
    DeError(#[from] quick_xml::DeError),
    #[error("Unable to write file")]
    IoError(#[from] std::io::Error),
}
