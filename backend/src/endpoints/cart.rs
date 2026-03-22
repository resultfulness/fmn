use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post, put},
};

use crate::{
    methods::cart::{
        add_item, add_recipe, read, read_events, redo, remove_item,
        reorder_items, undo, update_item,
    },
    models::{
        errors::APIError,
        requests::CartItemUpdateRequest,
        responses::{CartItem, EventResponse},
    },
    state::AppState,
};

pub fn get_cart_router() -> Router<AppState> {
    Router::new()
        .route("/", get(read_endpoint))
        .route("/events", get(read_events_endpoint))
        .route("/undo", post(undo_endpoint))
        .route("/redo", post(redo_endpoint))
        .route("/reorder", post(reorder_items_endpoint))
        .route("/recipe/{recipe_id}", post(add_recipe_endpoint))
        .route("/item/{item_id}", post(add_item_endpoint))
        .route("/item/{item_id}", put(update_item_endpoint))
        .route("/item/{item_id}", delete(remove_item_endpoint))
}

async fn undo_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(undo(&mut *queries).await?))
}

async fn redo_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(redo(&mut *queries).await?))
}
async fn add_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(add_item(&mut *queries, item_id).await?))
}
async fn remove_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(remove_item(&mut *queries, item_id).await?))
}
async fn update_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
    Json(request): Json<CartItemUpdateRequest>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(update_item(&mut *queries, item_id, request).await?))
}
async fn add_recipe_endpoint(
    State(state): State<AppState>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(add_recipe(&mut *queries, recipe_id).await?))
}
async fn reorder_items_endpoint(
    State(state): State<AppState>,
    Json(item_ids): Json<Vec<i32>>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(reorder_items(&mut *queries, item_ids).await?))
}
async fn read_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(read(&mut *queries).await?))
}
async fn read_events_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<EventResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(read_events(&mut *queries).await?))
}
