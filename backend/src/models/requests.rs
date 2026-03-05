use serde::Deserialize;

use crate::models::schema::RecipeItem;

#[derive(Deserialize, Debug, Clone)]
pub struct EchoRequest {
    pub message: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct ItemCreateRequest {
    pub name: String,
    pub icon: String,
    pub unit: String,
}
#[derive(Deserialize, Debug, Clone)]
pub struct ItemUpdateRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub unit: Option<String>,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RecipeCreateRequest {
    pub name: String,
    pub icon: String,
    pub description: String,
    pub servings: usize,
}
#[derive(Deserialize, Debug, Clone)]
pub struct RecipeUpdateRequest {
    pub name: Option<String>,
    pub icon: Option<String>,
    pub description: Option<String>,
    pub servings: Option<usize>,
    pub items: Option<Vec<RecipeItem>>,
}
