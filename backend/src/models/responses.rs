use axum::response::sse;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use uuid::Uuid;

use crate::models::schema::{Event, EventPayload, RecipeItem};

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct EchoResponse {
    pub message: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct Item {
    pub item_id: i32,
    pub name: String,
    pub icon: String,
    pub unit: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct CartItem {
    pub item_id: i32,
    pub description: Option<String>,
    pub quantity: Option<i32>,
}

impl CartItem {
    pub fn new(
        item_id: i32,
        description: Option<String>,
        quantity: Option<i32>,
    ) -> Self {
        Self {
            item_id,
            description,
            quantity,
        }
    }
}
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone, FromRow)]
pub struct Recipe {
    pub recipe_id: i32,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub servings: i32,
    pub items: Vec<RecipeItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct EventResponse {
    pub event_id: Uuid,
    pub event_type: String,
    pub is_future: bool,
    pub executed_at: DateTime<Utc>,
    pub payload: EventPayload,
}
impl From<Event> for EventResponse {
    fn from(value: Event) -> Self {
        Self {
            event_id: value.event_id,
            payload: value.payload.clone(),
            event_type: value.payload.into(),
            is_future: value.is_future,
            executed_at: value.executed_at,
        }
    }
}

pub struct StreamResponse(Vec<CartItem>);

impl StreamResponse {
    pub fn new(cart_items: Vec<CartItem>) -> Self {
        Self(cart_items)
    }
}

impl Into<sse::Event> for StreamResponse {
    fn into(self) -> sse::Event {
        sse::Event::default().data(serde_json::to_string(&self.0).unwrap())
    }
}
