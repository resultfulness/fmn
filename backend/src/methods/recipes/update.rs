use crate::{
    methods::queries::Queries,
    models::{
        errors::APIError, requests::RecipeUpdateRequest,
        responses::RecipeResponse,
    },
};

pub async fn update_recipe(
    queries: &mut impl Queries,
    recipe_id: usize,
    request: RecipeUpdateRequest,
) -> Result<RecipeResponse, APIError> {
    queries
        .recipe_select_one(recipe_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    if let Some(request_items) = &request.items {
        let items = queries.item_select_many().await?;
        for item in request_items {
            if let None = items.iter().find(|v| v.item_id == item.item_id) {
                return Err(APIError::NotFoundError);
            }
        }
    }
    queries.recipe_update_one(recipe_id, request).await?;
    let recipe = queries.recipe_select_one(recipe_id).await?;
    Ok(recipe.ok_or("could not update")?)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            items::create::create_item,
            memory_queries::MemoryQueries,
            recipes::{create::create_recipe, update::update_recipe},
        },
        models::{
            errors::APIError,
            requests::{
                ItemCreateRequest, RecipeCreateRequest, RecipeUpdateRequest,
            },
            schema::RecipeItem,
        },
    };

    #[tokio::test]
    async fn updates_recipe() {
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
        let recipe = create_recipe(
            &mut queries,
            RecipeCreateRequest {
                name: "foo".into(),
                icon: "".into(),
                description: "bar baz".into(),
                servings: 3,
            },
        )
        .await
        .unwrap();
        let result = update_recipe(
            &mut queries,
            recipe.recipe_id,
            RecipeUpdateRequest {
                name: Some("bar".into()),
                icon: Some("https://cataas.com/cat?id=1".into()),
                description: Some("baz qux".into()),
                servings: Some(4),
                items: Some(vec![RecipeItem {
                    item_id: item.item_id,
                    quantity: 5,
                }]),
            },
        )
        .await
        .unwrap();
        assert_eq!(result.name, "bar".to_owned());
        assert_eq!(result.icon, "https://cataas.com/cat?id=1".to_owned());
        assert_eq!(result.description, "baz qux".to_owned());
        assert_eq!(result.servings, 4);
    }
    #[tokio::test]
    async fn returns_item_not_found() {
        let mut queries = MemoryQueries::default();
        let recipe = create_recipe(
            &mut queries,
            RecipeCreateRequest {
                name: "foo".into(),
                icon: "".into(),
                description: "bar baz".into(),
                servings: 3,
            },
        )
        .await
        .unwrap();
        let result = update_recipe(
            &mut queries,
            recipe.recipe_id,
            RecipeUpdateRequest {
                name: None,
                icon: None,
                description: None,
                servings: None,
                items: Some(vec![RecipeItem {
                    item_id: 0,
                    quantity: 5,
                }]),
            },
        )
        .await;
        assert_eq!(result, Err(APIError::NotFoundError));
    }
    #[tokio::test]
    async fn returns_not_found() {
        let mut queries = MemoryQueries::default();
        let result = update_recipe(
            &mut queries,
            0,
            RecipeUpdateRequest {
                name: None,
                icon: None,
                description: None,
                servings: None,
                items: None,
            },
        )
        .await;
        assert_eq!(result, Err(APIError::NotFoundError));
    }
}
