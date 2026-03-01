use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, post},
};

use crate::{
    methods::cart::{
        add_item::add_item, read::read, redo::redo, remove_item::remove_item,
        undo::undo,
    },
    models::{errors::APIError, responses::CartItemResponse},
    state::AppState,
};

pub fn get_cart_router() -> Router<AppState> {
    Router::new()
        .route("/", get(read_endpoint))
        .route("/undo", post(undo_endpoint))
        .route("/redo", post(redo_endpoint))
        .route("/item/{item_id}", post(add_item_endpoint))
        .route("/item/{item_id}", delete(remove_item_endpoint))
}

async fn undo_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItemResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(undo(&mut *queries).await?))
}

async fn redo_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItemResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(redo(&mut *queries).await?))
}
async fn add_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<usize>,
) -> Result<Json<Vec<CartItemResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(add_item(&mut *queries, item_id).await?))
}
async fn remove_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<usize>,
) -> Result<Json<Vec<CartItemResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(remove_item(&mut *queries, item_id).await?))
}
async fn read_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItemResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(read(&mut *queries).await?))
}
