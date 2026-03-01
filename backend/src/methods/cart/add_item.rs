use crate::{
    methods::queries::Queries,
    models::{errors::APIError, responses::CartItemResponse},
};

pub async fn add_item(
    queries: &mut impl Queries,
    item_id: usize,
) -> Result<Vec<CartItemResponse>, APIError> {
    queries
        .item_select_one(item_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    let items = queries.cart_item_select_all().await?;
    for item in &items {
        if item.item_id == item_id {
            return Ok(items);
        }
    }
    queries.cart_add_item(item_id).await?;
    let results = queries.cart_item_select_all().await?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            cart::add_item::add_item, items::create::create_item,
            memory_queries::MemoryQueries,
        },
        models::{errors::APIError, requests::ItemCreateRequest},
    };

    #[tokio::test]
    async fn returns_added_item_in_cart() {
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
        let result = add_item(&mut queries, item.item_id).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].item_id, item.item_id);
        assert_eq!(result[0].description, None);
        assert_eq!(result[0].quantity, None);
    }
    #[tokio::test]
    async fn returns_not_found() {
        let mut queries = MemoryQueries::default();
        let result = add_item(&mut queries, 0).await;
        assert_eq!(result, Err(APIError::NotFoundError));
    }
    #[tokio::test]
    async fn returns_one_if_added_twice() {
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
        add_item(&mut queries, item.item_id).await.unwrap();
        let result = add_item(&mut queries, item.item_id).await.unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].item_id, item.item_id);
        assert_eq!(result[0].description, None);
        assert_eq!(result[0].quantity, None);
    }
}
