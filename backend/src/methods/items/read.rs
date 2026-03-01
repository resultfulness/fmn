use crate::{
    methods::queries::Queries,
    models::{errors::APIError, responses::ItemResponse},
};

pub async fn read_item(
    queries: &mut impl Queries,
    item_id: usize,
) -> Result<ItemResponse, APIError> {
    let item = queries
        .item_select_one(item_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    Ok(item)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            items::{create::create_item, read::read_item},
            memory_queries::MemoryQueries,
        },
        models::{errors::APIError, requests::ItemCreateRequest},
    };

    #[tokio::test]
    async fn returns_item() {
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
        let result = read_item(&mut queries, item.item_id).await.unwrap();
        assert_eq!(result.name, "foo".to_owned());
        assert_eq!(result.icon, "".to_owned());
        assert_eq!(result.unit, "tbsp".to_owned());
    }
    #[tokio::test]
    async fn returns_not_found() {
        let mut queries = MemoryQueries::default();
        let result = read_item(&mut queries, 0).await;
        assert_eq!(result, Err(APIError::NotFoundError));
    }
}
