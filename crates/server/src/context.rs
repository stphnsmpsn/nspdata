use crate::nspdata::cache::in_memory_cache::InMemoryCache;
use crate::nspdata::cache::NspCache;
use std::sync::Arc;

pub struct Context {
    nsp_cache: Arc<dyn NspCache + Send + Sync>,
}

impl Context {
    pub(crate) fn new_with_in_memory_cache() -> Self {
        Self {
            nsp_cache: Arc::new(InMemoryCache::default()),
        }
    }

    pub(crate) fn nsp_cache(&self) -> Arc<dyn NspCache + Send + Sync> {
        self.nsp_cache.clone()
    }
}
