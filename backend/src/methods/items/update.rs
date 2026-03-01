use crate::{
    methods::queries::Queries,
    models::{
        errors::APIError, requests::ItemUpdateRequest, responses::ItemResponse,
    },
};

pub async fn update_item(
    queries: &mut impl Queries,
    item_id: usize,
    request: ItemUpdateRequest,
) -> Result<ItemResponse, APIError> {
    queries
        .item_select_one(item_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    queries.item_update_one(item_id, request).await?;
    let item = queries.item_select_one(item_id).await?;
    Ok(item.ok_or("could not update")?)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            items::{create::create_item, update::update_item},
            memory_queries::MemoryQueries,
        },
        models::{
            errors::APIError,
            requests::{ItemCreateRequest, ItemUpdateRequest},
        },
    };

    #[tokio::test]
    async fn updates_item() {
        let mut queries = MemoryQueries::default();
        let item = create_item(
            &mut queries,
            ItemCreateRequest {
                name: "foo".into(),
                icon: "".into(),
                unit: "tbsp".into(),
            },
        )
        .await
        .unwrap();
        let result = update_item(
            &mut queries,
            item.item_id,
            ItemUpdateRequest {
                name: Some("bar".into()),
                icon: Some("https://cataas.com/cat?id=1".into()),
                unit: Some("tsp".into()),
            },
        )
        .await
        .unwrap();
        assert_eq!(result.name, "bar".to_owned());
        assert_eq!(result.icon, "https://cataas.com/cat?id=1".to_owned());
        assert_eq!(result.unit, "tsp".to_owned());
    }
    #[tokio::test]
    async fn returns_not_found() {
        let mut queries = MemoryQueries::default();
        let result = update_item(
            &mut queries,
            0,
            ItemUpdateRequest {
                name: None,
                icon: None,
                unit: None,
            },
        )
        .await;
        assert_eq!(result, Err(APIError::NotFoundError));
    }
}
