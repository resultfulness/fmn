use crate::{
    methods::queries::Queries,
    models::{
        errors::APIError, requests::ItemCreateRequest, responses::ItemResponse,
    },
};

pub async fn create_item(
    queries: &mut impl Queries,
    request: ItemCreateRequest,
) -> Result<ItemResponse, APIError> {
    let conflict = queries.item_select_one_by_name(&request.name).await?;
    if let Some(_) = conflict {
        return Err(APIError::AlreadyExistsError);
    }
    let item_id = queries.item_insert_one(request).await?;
    let item = queries.item_select_one(item_id).await?;
    Ok(item.ok_or("could not create")?)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{items::create::create_item, memory_queries::MemoryQueries},
        models::{errors::APIError, requests::ItemCreateRequest},
    };

    #[tokio::test]
    async fn returns_foo_when_foo() {
        let mut queries = MemoryQueries::default();
        let result = create_item(
            &mut queries,
            ItemCreateRequest {
                name: "foo".into(),
                icon: "".into(),
                unit: "tbsp".into(),
            },
        )
        .await
        .unwrap();
        assert_eq!(result.name, "foo".to_owned());
        assert_eq!(result.icon, "".to_owned());
        assert_eq!(result.unit, "tbsp".to_owned());
    }
    #[tokio::test]
    async fn returns_conflict_when_twice() {
        let mut queries = MemoryQueries::default();
        let request = ItemCreateRequest {
            name: "foo".into(),
            icon: "".into(),
            unit: "tbsp".into(),
        };
        create_item(&mut queries, request.clone()).await.unwrap();
        let result = create_item(&mut queries, request).await;
        assert_eq!(result, Err(APIError::AlreadyExistsError));
    }
}
