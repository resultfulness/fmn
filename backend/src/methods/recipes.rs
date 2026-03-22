use crate::{
    models::{
        errors::APIError,
        requests::{RecipeCreateRequest, RecipeUpdateRequest},
        responses::Recipe,
    },
    queries::Queries,
};

pub async fn create_recipe(
    queries: &Queries,
    request: RecipeCreateRequest,
) -> Result<Recipe, APIError> {
    let conflict = queries.recipe_select_one_by_name(&request.name).await?;
    if let Some(_) = conflict {
        return Err(APIError::AlreadyExistsError);
    }
    let recipe_id = queries.recipe_insert_one(request).await?;
    let recipe = queries.recipe_select_one(recipe_id).await?;
    Ok(recipe.ok_or("could not create")?)
}

pub async fn read_recipe(
    queries: &Queries,
    recipe_id: i32,
) -> Result<Recipe, APIError> {
    let recipe = queries
        .recipe_select_one(recipe_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    Ok(recipe)
}

pub async fn update_recipe(
    queries: &Queries,
    recipe_id: i32,
    request: RecipeUpdateRequest,
) -> Result<Recipe, APIError> {
    queries
        .recipe_select_one(recipe_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    if let Some(request_items) = &request.items {
        let items = queries.item_select_many().await?;
        if request_items.iter().any(|item| !items.contains_key(&item.item_id)) {
            return Err(APIError::NotFoundError);
        }
    }
    queries.recipe_update_one(recipe_id, request).await?;
    let recipe = queries.recipe_select_one(recipe_id).await?;
    Ok(recipe.ok_or("could not update")?)
}

pub async fn delete_recipe(
    queries: &Queries,
    recipe_id: i32,
) -> Result<Recipe, APIError> {
    let recipe = queries
        .recipe_select_one(recipe_id)
        .await?
        .ok_or(APIError::NotFoundError)?;
    queries.recipe_delete_one(recipe_id).await?;
    Ok(recipe)
}

pub async fn search_recipes(
    queries: &Queries,
) -> Result<Vec<Recipe>, APIError> {
    let recipes = queries.recipe_select_many().await?;
    let mut recipes: Vec<Recipe> =
        recipes.into_iter().map(|(_, v)| v).collect();
    recipes.sort_by(|a, b| a.recipe_id.cmp(&b.recipe_id));
    Ok(recipes)
}
