use crate::{
    methods::queries::Queries,
    models::{errors::APIError, responses::CartItemResponse},
};

pub async fn undo(
    queries: &mut impl Queries,
) -> Result<Vec<CartItemResponse>, APIError> {
    let (actions, _) = queries.cart_select_actions().await?;
    if actions.len() < 1 {
        let results = queries.cart_item_select_all().await?;
        return Ok(results);
    }
    queries.cart_undo().await?;
    let results = queries.cart_item_select_all().await?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            cart::{add_item::add_item, remove_item::remove_item, undo::undo},
            items::create::create_item,
            memory_queries::MemoryQueries,
        },
        models::requests::ItemCreateRequest,
    };

    #[tokio::test]
    async fn returns_undone() {
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
        remove_item(&mut queries, item.item_id).await.unwrap();
        let result = undo(&mut queries).await.unwrap();
        assert_eq!(result.len(), 1);
    }
    #[tokio::test]
    async fn returns_empty_if_no_undo() {
        let mut queries = MemoryQueries::default();
        let result = undo(&mut queries).await.unwrap();
        assert_eq!(result.len(), 0);
    }
}
