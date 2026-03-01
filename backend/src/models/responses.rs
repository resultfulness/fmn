use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct EchoResponse {
    pub message: String,
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

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct CartItemResponse {
    pub item_id: usize,
    pub description: Option<String>,
    pub quantity: Option<usize>,
}
