use crate::nspdata::cache::NspCache;
use crate::ServerError;
use bidgely_adapter::session::SessionResponse;
use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::RwLock;

#[derive(Default)]
pub struct InMemoryCache {
    user_id_sessions: RwLock<HashMap<String, SessionResponse>>,
    hits: AtomicU64,
    requests: AtomicU64,
}

#[async_trait]
impl NspCache for InMemoryCache {
    async fn find_session(&self, user_id: &String) -> Result<Option<SessionResponse>, ServerError> {
        self.requests.fetch_add(1, Ordering::Relaxed);
        let map = self.user_id_sessions.read()?;
        Ok(match map.get(user_id).cloned() {
            Some(session_response) => {
                self.hits.fetch_add(1, Ordering::Relaxed);
                Some(session_response)
            }
            None => None,
        })
    }

    async fn store_session(
        &self,
        user_id: String,
        session: SessionResponse,
    ) -> Result<Option<SessionResponse>, crate::ServerError> {
        {
            self.user_id_sessions
                .write()?
                .insert(user_id.clone(), session);
        }
        self.find_session(&user_id).await
    }

    fn hits(&self) -> u64 {
        self.hits.load(Ordering::Relaxed)
    }

    fn requests(&self) -> u64 {
        self.requests.load(Ordering::Relaxed)
    }
}
