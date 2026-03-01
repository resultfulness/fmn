use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct EchoResponse {
    pub message: String,
}
impl EchoResponse {
    pub fn new(message: &str) -> Self {
        Self {
            message: message.into(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct ItemResponse {
    pub item_id: usize,
    pub name: String,
    pub icon: String,
    pub unit: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
