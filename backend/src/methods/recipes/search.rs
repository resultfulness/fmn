use crate::{
    methods::queries::Queries,
    models::{errors::APIError, responses::RecipeResponse},
};

pub async fn search_recipes(
    queries: &mut impl Queries,
) -> Result<Vec<RecipeResponse>, APIError> {
    let recipes = queries.recipe_select_many().await?;
    Ok(recipes)
}

#[cfg(test)]
mod tests {
    use crate::{
        methods::{
            memory_queries::MemoryQueries,
            recipes::{
                create::create_recipe, read::read_recipe,
                search::search_recipes,
            },
        },
        models::{errors::APIError, requests::RecipeCreateRequest},
    };

    #[tokio::test]
    async fn returns_recipes() {
        let mut queries = MemoryQueries::default();
        create_recipe(
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
        create_recipe(
            &mut queries,
            RecipeCreateRequest {
                name: "bar".into(),
                icon: "https://cataas.com/cat?id=2".into(),
                description: "bar baz".into(),
                servings: 3,
            },
        )
        .await
        .unwrap();
        let results = search_recipes(&mut queries).await;
        let results = results.unwrap();
        assert_eq!(results.len(), 2);
        assert_eq!(results[0].name, "foo".to_owned());
        assert_eq!(results[1].name, "bar".to_owned());
    }
    #[tokio::test]
    async fn returns_not_found() {
        let mut queries = MemoryQueries::default();
        let result = read_recipe(&mut queries, 0).await;
        assert_eq!(result, Err(APIError::NotFoundError));
    }
}
