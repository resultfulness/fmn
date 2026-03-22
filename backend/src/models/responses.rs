use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

use crate::models::schema::RecipeItem;

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
