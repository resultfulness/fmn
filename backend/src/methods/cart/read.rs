use crate::{
    methods::queries::Queries,
    models::{errors::APIError, responses::CartItemResponse},
};

pub async fn read(
    queries: &mut impl Queries,
) -> Result<Vec<CartItemResponse>, APIError> {
    let results = queries.cart_item_select_all().await?;
    Ok(results)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            cart::{add_item::add_item, read::read},
            items::create::create_item,
            memory_queries::MemoryQueries,
        },
        models::requests::ItemCreateRequest,
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
        let response = read(&mut queries).await.unwrap();
        assert_eq!(result, response);
    }
}
