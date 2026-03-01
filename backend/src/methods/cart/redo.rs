use crate::{
    methods::queries::Queries,
    models::{errors::APIError, responses::CartItemResponse},
};

pub async fn redo(
    queries: &mut impl Queries,
) -> Result<Vec<CartItemResponse>, APIError> {
    let (_, actions_undone) = queries.cart_select_actions().await?;
    if actions_undone.len() < 1 {
        let results = queries.cart_item_select_all().await?;
        return Ok(results);
    }
    queries.cart_redo().await?;
    let results = queries.cart_item_select_all().await?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            cart::{
                add_item::add_item, redo::redo, remove_item::remove_item,
                undo::undo,
            },
            items::create::create_item,
            memory_queries::MemoryQueries,
        },
        models::requests::ItemCreateRequest,
    };

    #[tokio::test]
    async fn returns_redone() {
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
        undo(&mut queries).await.unwrap();
        undo(&mut queries).await.unwrap();
        let result = redo(&mut queries).await.unwrap();
        assert_eq!(result.len(), 1);
    }
    #[tokio::test]
    async fn returns_the_same_if_redo_full() {
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
        let result = redo(&mut queries).await.unwrap();
        assert_eq!(result.len(), 1);
    }
}
