use bidgely_adapter::session::SessionResponse;

pub(crate) mod in_memory_cache;

#[async_trait]
pub(crate) trait NspCache {
    async fn find_session(
        &self,
        user_id: &String,
    ) -> Result<Option<SessionResponse>, crate::ServerError>;
    async fn store_session(
        &self,
        user_id: String,
        session: SessionResponse,
    ) -> Result<Option<SessionResponse>, crate::ServerError>;
    fn hits(&self) -> u64;
    fn requests(&self) -> u64;
}
