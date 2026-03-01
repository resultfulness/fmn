use crate::{
    methods::queries::Queries,
    models::{errors::APIError, responses::ItemResponse},
};

pub async fn search_items(
    queries: &mut impl Queries,
) -> Result<Vec<ItemResponse>, APIError> {
    let items = queries.item_select_many().await?;
    Ok(items)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            items::{
                create::create_item, read::read_item, search::search_items,
            },
            memory_queries::MemoryQueries,
        },
        models::{errors::APIError, requests::ItemCreateRequest},
    };

    #[tokio::test]
    async fn returns_items() {
        let mut queries = MemoryQueries::default();
        create_item(
            &mut queries,
            ItemCreateRequest {
                name: "foo".into(),
                icon: "".into(),
                unit: "tbsp".into(),
            },
        )
        .await
        .unwrap();
        create_item(
            &mut queries,
            ItemCreateRequest {
                name: "bar".into(),
                icon: "https://cataas.com/cat?id=2".into(),
                unit: "tsp".into(),
            },
        )
        .await
        .unwrap();
        let results = search_items(&mut queries).await;
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "foo".to_owned());
        assert_eq!(results[1].name, "bar".to_owned());
    }
    #[tokio::test]
    async fn returns_not_found() {
        let mut queries = MemoryQueries::default();
        let result = read_item(&mut queries, 0).await;
        assert_eq!(result, Err(APIError::NotFoundError));
    }
}
