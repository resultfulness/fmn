use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::{Value, from_value};
use sqlx::prelude::{FromRow, Type};
use uuid::Uuid;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
pub enum EventPayload {
    AddItem(i32),
    AddItems(Vec<(i32, i32)>),
    RemoveItem(i32),
    EditItem(i32, Option<String>, Option<i32>),
    Reorder(Vec<i32>),
}
impl From<EventPayload> for String {
    fn from(value: EventPayload) -> Self {
        match value {
            EventPayload::AddItem(..) => "AddItem",
            EventPayload::AddItems(..) => "AddItems",
            EventPayload::RemoveItem(..) => "RemoveItem",
            EventPayload::EditItem(..) => "EditItem",
            EventPayload::Reorder(..) => "Reorder",
        }
        .into()
    }
}

impl From<Value> for EventPayload {
    fn from(value: Value) -> Self {
        from_value::<Self>(value).unwrap()
    }
}

#[derive(Debug, PartialEq, Eq, Clone, FromRow)]
pub struct Event {
    pub event_id: Uuid,
    pub payload: EventPayload,
    pub is_future: bool,
    pub executed_at: DateTime<Utc>,
}

#[derive(
    Debug, PartialEq, Eq, Deserialize, Serialize, Clone, FromRow, Type,
)]
pub struct RecipeItem {
    pub item_id: i32,
    pub quantity: i32,
}
