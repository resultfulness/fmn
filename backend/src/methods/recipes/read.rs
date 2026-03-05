use crate::{
    methods::queries::Queries,
    models::{errors::APIError, responses::RecipeResponse},
};

pub async fn read_recipe(
    queries: &mut impl Queries,
    recipe_id: usize,
) -> Result<RecipeResponse, APIError> {
    let recipe = queries
        .recipe_select_one(recipe_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    Ok(recipe)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            recipes::{create::create_recipe, read::read_recipe},
            memory_queries::MemoryQueries,
        },
        models::{errors::APIError, requests::RecipeCreateRequest},
    };

    #[tokio::test]
    async fn returns_recipe() {
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
        let result = read_recipe(&mut queries, recipe.recipe_id).await.unwrap();
        assert_eq!(result.name, "foo".to_owned());
        assert_eq!(result.icon, "".to_owned());
        assert_eq!(result.description, "bar baz".to_owned());
        assert_eq!(result.servings, 3);
    }
    #[tokio::test]
    async fn returns_not_found() {
        let mut queries = MemoryQueries::default();
        let result = read_recipe(&mut queries, 0).await;
        assert_eq!(result, Err(APIError::NotFoundError));
    }
}
