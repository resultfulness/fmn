use crate::{
    models::{
        errors::APIError,
        requests::{ItemCreateRequest, ItemUpdateRequest},
        responses::Item,
    },
    queries::Queries,
};

pub async fn create_item(
    queries: &Queries,
    request: ItemCreateRequest,
) -> Result<Item, APIError> {
    let conflict = queries.item_select_one_by_name(&request.name).await?;
    if let Some(_) = conflict {
        return Err(APIError::AlreadyExistsError);
    }
    let item_id = queries.item_insert_one(request).await?;
    let item = queries.item_select_one(item_id).await?;
    Ok(item.ok_or("could not create")?)
}

pub async fn read_item(
    queries: &Queries,
    item_id: i32,
) -> Result<Item, APIError> {
    let item = queries
        .item_select_one(item_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    Ok(item)
}

pub async fn update_item(
    queries: &Queries,
    item_id: i32,
    request: ItemUpdateRequest,
) -> Result<Item, APIError> {
    queries.item_select_one(item_id).await?.ok_or(APIError::NotFoundError)?;
    queries.item_update_one(item_id, request).await?;
    let item = queries.item_select_one(item_id).await?;
    Ok(item.ok_or("could not update")?)
}

pub async fn delete_item(
    queries: &Queries,
    item_id: i32,
) -> Result<Item, APIError> {
    let item = queries
        .item_select_one(item_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    queries.item_delete_one(item_id).await?;
    Ok(item)
}

pub async fn search_items(queries: &Queries) -> Result<Vec<Item>, APIError> {
    let items = queries.item_select_many().await?;
    let mut items: Vec<Item> = items.into_iter().map(|(_, v)| v).collect();
    items.sort_by(|a, b| a.item_id.cmp(&b.item_id));
    Ok(items)
}
