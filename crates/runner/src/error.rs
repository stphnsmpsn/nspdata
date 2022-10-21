#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Bidgely Error")]
    BidgelyError(#[from] bidgely_adapter::BidgelyError),
    #[error("Serde JSON Error")]
    SerdeJson(#[from] serde_json::Error),
    #[error("Quick XML De Error")]
    DeError(#[from] quick_xml::DeError),
    #[error("IO Error")]
    IoError(#[from] std::io::Error),
    #[error("Bad Argument: {0}")]
    BadArgument(String),
    #[error("Server Error")]
    ServerError(#[from] server::error::ServerError),
}
