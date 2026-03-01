use std::sync::Arc;

use tokio::sync::Mutex;

use crate::methods::memory_queries::MemoryQueries;

#[derive(Clone)]
pub struct AppState {
    pub queries: Arc<Mutex<MemoryQueries>>,
}
