use bidgely_adapter::session::SessionResponse;
use std::collections::HashMap;
use std::sync::{PoisonError, RwLockReadGuard, RwLockWriteGuard};

#[derive(thiserror::Error, Debug)]
pub enum ServerError {
    #[error("Rocket Error {0}")]
    RocketError(#[from] rocket::error::Error),
    #[error("Bidgely Error {0}")]
    BidgelyError(#[from] bidgely_adapter::BidgelyError),
    #[error("Read Guard Poisoned")]
    ReadGuardPoisoned,
    #[error("Write Guard Poisoned")]
    WriteGuardPoisoned,
    #[error("SerdeJsonError")]
    SerdeJsonError(#[from] serde_json::Error),
}

impl From<PoisonError<RwLockReadGuard<'_, HashMap<String, SessionResponse>>>> for ServerError {
    fn from(_: PoisonError<RwLockReadGuard<'_, HashMap<String, SessionResponse>>>) -> Self {
        Self::ReadGuardPoisoned
    }
}

impl From<PoisonError<RwLockWriteGuard<'_, HashMap<String, SessionResponse>>>> for ServerError {
    fn from(_: PoisonError<RwLockWriteGuard<'_, HashMap<String, SessionResponse>>>) -> Self {
        Self::WriteGuardPoisoned
    }
}
