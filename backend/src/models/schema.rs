use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CartEvent {
    AddItem { item_id: usize },
    RemoveItem { item_id: usize },
    Undo,
    Redo,
}

#[derive(Debug, PartialEq, Eq, Deserialize, Serialize, Clone)]
pub struct RecipeItem {
    pub item_id: usize,
    pub quantity: usize,
}
