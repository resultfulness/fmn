use std::collections::HashMap;

use crate::{
    models::{
        errors::APIError, requests::CartItemUpdateRequest, responses::CartItem,
        schema::EventPayload,
    },
    queries::Queries,
};

async fn update_state(queries: &Queries) -> Result<(), APIError> {
    let events = queries.cart_select_many_event().await?;
    let mut ordering: Vec<i32> = vec![];
    let mut items: HashMap<i32, CartItem> = HashMap::new();
    for event in events {
        if event.is_future {
            break;
        }
        match event.payload {
            EventPayload::AddItem(item_id) => {
                ordering.push(item_id);
                items.insert(item_id, CartItem::new(item_id, None, None));
            }
            EventPayload::RemoveItem(item_id) => {
                ordering.retain(|v| *v != item_id);
                items.remove(&item_id);
            }
            EventPayload::EditItem(item_id, description, quantity) => {
                let item = items.get_mut(&item_id).unwrap();
                item.description = description;
                item.quantity = quantity;
            }
            EventPayload::AddItems(item_ids) => {
                for (item_id, quantity) in item_ids {
                    if let Some(item) = items.get_mut(&item_id) {
                        item.quantity =
                            Some(item.quantity.unwrap_or(1) + quantity);
                    } else {
                        ordering.push(item_id);
                        items.insert(
                            item_id,
                            CartItem::new(item_id, None, Some(quantity)),
                        );
                    }
                }
            }
            EventPayload::Reorder(new_ordering) => ordering = new_ordering,
        }
    }
    let items = ordering.iter().filter_map(|v| items.remove(v)).collect();
    queries.cart_delete_many_item().await?;
    queries.cart_insert_many_item(items).await?;
    Ok(())
}

pub async fn add_item(
    queries: &Queries,
    item_id: i32,
) -> Result<Vec<CartItem>, APIError> {
    queries.item_select_one(item_id).await?.ok_or(APIError::NotFoundError)?;
    let items = queries.cart_select_many_item().await?;
    if let Some(_) = items.iter().find(|v| v.item_id == item_id) {
        return Ok(items);
    }
    queries.cart_insert_one_event(EventPayload::AddItem(item_id)).await?;
    queries.cart_delete_future_event().await?;
    update_state(queries).await?;
    Ok(queries.cart_select_many_item().await?)
}

pub async fn add_recipe(
    queries: &Queries,
    recipe_id: i32,
) -> Result<Vec<CartItem>, APIError> {
    let recipe = queries
        .recipe_select_one(recipe_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    queries
        .cart_insert_one_event(EventPayload::AddItems(
            recipe.items.into_iter().map(|v| (v.item_id, v.quantity)).collect(),
        ))
        .await?;
    queries.cart_delete_future_event().await?;
    update_state(queries).await?;
    Ok(queries.cart_select_many_item().await?)
}

pub async fn update_item(
    queries: &Queries,
    item_id: i32,
    request: CartItemUpdateRequest,
) -> Result<Vec<CartItem>, APIError> {
    queries.item_select_one(item_id).await?.ok_or(APIError::NotFoundError)?;
    queries
        .cart_insert_one_event(EventPayload::EditItem(
            item_id,
            request.description,
            request.quantity,
        ))
        .await?;
    queries.cart_delete_future_event().await?;
    update_state(queries).await?;
    Ok(queries.cart_select_many_item().await?)
}

pub async fn reorder_items(
    queries: &Queries,
    item_ids: Vec<i32>,
) -> Result<Vec<CartItem>, APIError> {
    let items = queries.item_select_many().await?;
    for item_id in &item_ids {
        if let None = items.iter().find(|v| v.item_id == *item_id) {
            return Err(APIError::NotFoundError);
        }
    }
    queries.cart_insert_one_event(EventPayload::Reorder(item_ids)).await?;
    queries.cart_delete_future_event().await?;
    update_state(queries).await?;
    Ok(queries.cart_select_many_item().await?)
}

pub async fn remove_item(
    queries: &Queries,
    item_id: i32,
) -> Result<Vec<CartItem>, APIError> {
    queries.item_select_one(item_id).await?.ok_or(APIError::NotFoundError)?;
    let items = queries.cart_select_many_item().await?;
    if let None = items.iter().find(|v| v.item_id == item_id) {
        return Ok(items);
    }
    queries.cart_insert_one_event(EventPayload::RemoveItem(item_id)).await?;
    queries.cart_delete_future_event().await?;
    update_state(queries).await?;
    Ok(queries.cart_select_many_item().await?)
}

pub async fn read(queries: &Queries) -> Result<Vec<CartItem>, APIError> {
    Ok(queries.cart_select_many_item().await?)
}

pub async fn redo(queries: &Queries) -> Result<Vec<CartItem>, APIError> {
    let events = queries.cart_select_many_event().await?;
    let items = queries.cart_select_many_item().await?;
    let Some(event) = events.iter().find(|v| v.is_future) else {
        return Ok(items);
    };
    queries.cart_update_one_event(event.event_id, false).await?;
    update_state(queries).await?;
    Ok(queries.cart_select_many_item().await?)
}

pub async fn undo(queries: &Queries) -> Result<Vec<CartItem>, APIError> {
    let events = queries.cart_select_many_event().await?;
    let items = queries.cart_select_many_item().await?;
    if events.len() < 1 {
        return Ok(items);
    }
    let Some(event) = events.iter().take_while(|v| !v.is_future).last() else {
        return Ok(items);
    };
    queries.cart_update_one_event(event.event_id, true).await?;
    update_state(queries).await?;
    Ok(queries.cart_select_many_item().await?)
}
