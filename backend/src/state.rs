use std::sync::Arc;

use tokio::sync::Mutex;

use crate::queries::Queries;

#[derive(Clone)]
pub struct AppState {
    pub queries: Arc<Mutex<Queries>>,
}
