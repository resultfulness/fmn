use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::schema::RecipeItem;

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
#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct RecipeResponse {
    pub recipe_id: usize,
    pub name: String,
    pub icon: String,
    pub description: String,
    pub servings: usize,
    pub items: Vec<RecipeItem>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
