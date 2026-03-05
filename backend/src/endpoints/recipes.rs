use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, patch, post},
};

use crate::{
    methods::recipes::{
        create::create_recipe, delete::delete_recipe, read::read_recipe,
        search::search_recipes, update::update_recipe,
    },
    models::{
        errors::APIError,
        requests::{RecipeCreateRequest, RecipeUpdateRequest},
        responses::RecipeResponse,
    },
    state::AppState,
};

pub fn get_recipes_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_recipe_endpoint))
        .route("/", get(search_recipes_endpoint))
        .route("/{recipe_id}", get(read_recipe_endpoint))
        .route("/{recipe_id}", patch(update_recipe_endpoint))
        .route("/{recipe_id}", delete(delete_recipe_endpoint))
}

async fn search_recipes_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<RecipeResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(search_recipes(&mut *queries).await?))
}

async fn create_recipe_endpoint(
    State(state): State<AppState>,
    Json(request): Json<RecipeCreateRequest>,
) -> Result<Json<RecipeResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(create_recipe(&mut *queries, request).await?))
}

async fn read_recipe_endpoint(
    State(state): State<AppState>,
    Path(recipe_id): Path<usize>,
) -> Result<Json<RecipeResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(read_recipe(&mut *queries, recipe_id).await?))
}

async fn update_recipe_endpoint(
    State(state): State<AppState>,
    Path(recipe_id): Path<usize>,
    Json(request): Json<RecipeUpdateRequest>,
) -> Result<Json<RecipeResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(update_recipe(&mut *queries, recipe_id, request).await?))
}

async fn delete_recipe_endpoint(
    State(state): State<AppState>,
    Path(recipe_id): Path<usize>,
) -> Result<Json<RecipeResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(delete_recipe(&mut *queries, recipe_id).await?))
}
