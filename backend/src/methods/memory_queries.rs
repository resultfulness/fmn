use std::collections::HashMap;

use chrono::Utc;

use crate::{
    methods::queries::Queries,
    models::{
        requests::{
            ItemCreateRequest, ItemUpdateRequest, RecipeCreateRequest,
            RecipeUpdateRequest,
        },
        responses::{CartItemResponse, ItemResponse, RecipeResponse},
        schema::CartEvent,
    },
};

#[derive(Default)]
pub struct MemoryQueries {
    max_item_id: usize,
    items: HashMap<usize, ItemResponse>,
    max_recipe_id: usize,
    recipes: HashMap<usize, RecipeResponse>,

    cart_items: Option<Vec<CartItemResponse>>,
    cart_events: Vec<CartEvent>,
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

    async fn cart_add_item(&mut self, item_id: usize) -> Result<(), String> {
        self.cart_events.push(CartEvent::AddItem { item_id });
        self.cart_items = None;
        Ok(())
    }
    async fn cart_remove_item(&mut self, item_id: usize) -> Result<(), String> {
        self.cart_events.push(CartEvent::RemoveItem { item_id });
        self.cart_items = None;
        Ok(())
    }
    async fn cart_undo(&mut self) -> Result<(), String> {
        self.cart_events.push(CartEvent::Undo);
        self.cart_items = None;
        Ok(())
    }
    async fn cart_redo(&mut self) -> Result<(), String> {
        self.cart_events.push(CartEvent::Redo);
        self.cart_items = None;
        Ok(())
    }
    async fn cart_select_actions(
        &self,
    ) -> Result<(Vec<CartEvent>, Vec<CartEvent>), String> {
        let mut actions: Vec<CartEvent> = vec![];
        let mut actions_undone: Vec<CartEvent> = vec![];

        for event in &self.cart_events {
            match event {
                CartEvent::Undo => {
                    actions_undone.push(actions.pop().ok_or("can't undo")?)
                }
                CartEvent::Redo => {
                    actions.push(actions_undone.pop().ok_or("can't redo")?)
                }
                v => actions.push(v.clone()),
            }
        }
        Ok((actions, actions_undone))
    }
    async fn cart_item_select_all(
        &mut self,
    ) -> Result<Vec<CartItemResponse>, String> {
        if let Some(cart_items) = &self.cart_items {
            return Ok(cart_items.clone());
        }
        let (actions, _) = self.cart_select_actions().await?;

        let mut items: Vec<CartItemResponse> = vec![];
        for action in actions {
            match action {
                CartEvent::AddItem { item_id } => {
                    items.push(CartItemResponse {
                        item_id: item_id,
                        description: None,
                        quantity: None,
                    })
                }
                CartEvent::RemoveItem { item_id } => {
                    let item_idx = items
                        .iter()
                        .position(|v| v.item_id == item_id)
                        .ok_or("not found")?;
                    items.remove(item_idx);
                }
                _ => unreachable!(),
            }
        }
        self.cart_items = Some(items.clone());

        Ok(items)
    }

    async fn recipe_insert_one(
        &mut self,
        recipe: RecipeCreateRequest,
    ) -> Result<usize, String> {
        self.max_recipe_id += 1;
        self.recipes.insert(
            self.max_recipe_id,
            RecipeResponse {
                recipe_id: self.max_recipe_id,
                name: recipe.name,
                icon: recipe.icon,
                description: recipe.description,
                servings: recipe.servings,
                items: vec![],
                created_at: Utc::now(),
                updated_at: Utc::now(),
            },
        );
        Ok(self.max_recipe_id)
    }
    async fn recipe_update_one(
        &mut self,
        recipe_id: usize,
        recipe: RecipeUpdateRequest,
    ) -> Result<(), String> {
        let inner_recipe =
            self.recipes.get_mut(&recipe_id).ok_or("not found")?;
        if let Some(name) = recipe.name {
            inner_recipe.name = name;
        }
        if let Some(icon) = recipe.icon {
            inner_recipe.icon = icon;
        }
        if let Some(description) = recipe.description {
            inner_recipe.description = description;
        }
        if let Some(servings) = recipe.servings {
            inner_recipe.servings = servings;
        }
        if let Some(items) = recipe.items {
            inner_recipe.items = items;
        }
        Ok(())
    }
    async fn recipe_select_one(
        &self,
        recipe_id: usize,
    ) -> Result<Option<RecipeResponse>, String> {
        Ok(self.recipes.get(&recipe_id).map(|v| v.clone()))
    }
    async fn recipe_select_one_by_name(
        &self,
        name: &str,
    ) -> Result<Option<usize>, String> {
        Ok(self
            .recipes
            .iter()
            .filter(|(_, recipe)| recipe.name == name)
            .map(|(recipe_id, _)| *recipe_id)
            .next())
    }
    async fn recipe_delete_one(
        &mut self,
        recipe_id: usize,
    ) -> Result<(), String> {
        self.recipes.remove(&recipe_id).ok_or("not found")?;
        Ok(())
    }
    async fn recipe_select_many(&self) -> Result<Vec<RecipeResponse>, String> {
        let mut v: Vec<RecipeResponse> = self
            .recipes
            .values()
            .into_iter()
            .map(|v| v.clone())
            .collect();
        v.sort_by(|a, b| a.recipe_id.cmp(&b.recipe_id));
        Ok(v)
    }
}
