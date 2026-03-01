#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CartEvent {
    AddItem { item_id: usize },
    RemoveItem { item_id: usize },
    Undo,
    Redo,
}
