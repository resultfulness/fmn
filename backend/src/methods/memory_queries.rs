use std::collections::HashMap;

use chrono::Utc;

use crate::{
    methods::queries::Queries,
    models::{
        requests::{ItemCreateRequest, ItemUpdateRequest},
        responses::ItemResponse,
    },
};

#[derive(Default)]
pub struct MemoryQueries {
    max_item_id: usize,
    items: HashMap<usize, ItemResponse>,
}

impl Queries for MemoryQueries {
    async fn item_insert_one(
        &mut self,
        item: ItemCreateRequest,
    ) -> Result<usize, String> {
        self.max_item_id += 1;
        self.items.insert(
            self.max_item_id,
            ItemResponse {
                item_id: self.max_item_id,
                name: item.name,
                icon: item.icon,
                unit: item.unit,
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        );
        Ok(self.max_item_id)
    }
    async fn item_update_one(
        &mut self,
        item_id: usize,
        item: ItemUpdateRequest,
    ) -> Result<(), String> {
        let inner_item = self.items.get_mut(&item_id).ok_or("not found")?;
        if let Some(name) = item.name {
            inner_item.name = name;
        }
        if let Some(icon) = item.icon {
            inner_item.icon = icon;
        }
        if let Some(unit) = item.unit {
            inner_item.unit = unit;
        }
        Ok(())
    }
    async fn item_select_one(
        &self,
        item_id: usize,
    ) -> Result<Option<ItemResponse>, String> {
        Ok(self.items.get(&item_id).map(|v| v.clone()))
    }
    async fn item_select_one_by_name(
        &self,
        name: &str,
    ) -> Result<Option<usize>, String> {
        Ok(self
            .items
            .iter()
            .filter(|(_, item)| item.name == name)
            .map(|(item_id, _)| *item_id)
            .next())
    }
    async fn item_delete_one(&mut self, item_id: usize) -> Result<(), String> {
        self.items.remove(&item_id).ok_or("not found")?;
        Ok(())
    }
    async fn item_select_many(&self) -> Result<Vec<ItemResponse>, String> {
        let mut v: Vec<ItemResponse> =
            self.items.values().into_iter().map(|v| v.clone()).collect();
        v.sort_by(|a, b| a.item_id.cmp(&b.item_id));
        Ok(v)
    }
}
