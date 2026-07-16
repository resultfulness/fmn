use std::sync::Arc;

use axum::response::sse;
use tokio::sync::Mutex;
use tokio::sync::broadcast::Sender;

use crate::queries::Queries;

#[derive(Clone)]
pub struct AppState {
    pub queries: Arc<Mutex<Queries>>,
    pub tx: Sender<sse::Event>,
}
