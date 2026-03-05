use crate::{
    methods::queries::Queries,
    models::{
        errors::APIError, requests::RecipeCreateRequest,
        responses::RecipeResponse,
    },
};

pub async fn create_recipe(
    queries: &mut impl Queries,
    request: RecipeCreateRequest,
) -> Result<RecipeResponse, APIError> {
    let conflict = queries.recipe_select_one_by_name(&request.name).await?;
    if let Some(_) = conflict {
        return Err(APIError::AlreadyExistsError);
    }
    let recipe_id = queries.recipe_insert_one(request).await?;
    let recipe = queries.recipe_select_one(recipe_id).await?;
    Ok(recipe.ok_or("could not create")?)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            memory_queries::MemoryQueries, recipes::create::create_recipe,
        },
        models::{errors::APIError, requests::RecipeCreateRequest},
    };

    #[tokio::test]
    async fn returns_foo_when_foo() {
        let mut queries = MemoryQueries::default();
        let result = create_recipe(
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
        assert_eq!(result.name, "foo".to_owned());
        assert_eq!(result.icon, "".to_owned());
        assert_eq!(result.description, "bar baz".to_owned());
        assert_eq!(result.servings, 3);
    }
    #[tokio::test]
    async fn returns_conflict_when_twice() {
        let mut queries = MemoryQueries::default();
        let request = RecipeCreateRequest {
            name: "foo".into(),
            icon: "".into(),
            description: "bar baz".into(),
            servings: 3,
        };
        create_recipe(&mut queries, request.clone()).await.unwrap();
        let result = create_recipe(&mut queries, request).await;
        assert_eq!(result, Err(APIError::AlreadyExistsError));
    }
}
