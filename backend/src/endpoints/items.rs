use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, patch, post},
};

use crate::{
    methods::items::{
        create::create_item, delete::delete_item, read::read_item,
        search::search_items, update::update_item,
    },
    models::{
        errors::APIError,
        requests::{ItemCreateRequest, ItemUpdateRequest},
        responses::ItemResponse,
    },
    state::AppState,
};

pub fn get_items_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_item_endpoint))
        .route("/", get(search_items_endpoint))
        .route("/{item_id}", get(read_item_endpoint))
        .route("/{item_id}", patch(update_item_endpoint))
        .route("/{item_id}", delete(delete_item_endpoint))
}

async fn search_items_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<ItemResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(search_items(&mut *queries).await?))
}

async fn create_item_endpoint(
    State(state): State<AppState>,
    Json(request): Json<ItemCreateRequest>,
) -> Result<Json<ItemResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(create_item(&mut *queries, request).await?))
}

async fn read_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<usize>,
) -> Result<Json<ItemResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(read_item(&mut *queries, item_id).await?))
}

async fn update_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<usize>,
    Json(request): Json<ItemUpdateRequest>,
) -> Result<Json<ItemResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(update_item(&mut *queries, item_id, request).await?))
}

async fn delete_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<usize>,
) -> Result<Json<ItemResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(delete_item(&mut *queries, item_id).await?))
}
