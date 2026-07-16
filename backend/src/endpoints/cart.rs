use axum::{
    Json, Router,
    extract::{Path, State},
    response::{Sse, sse},
    routing::{delete, get, post, put},
};
use futures_util::Stream;
use tokio_stream::wrappers::{
    BroadcastStream, errors::BroadcastStreamRecvError,
};

use crate::{
    methods::cart::{
        add_item, add_recipe, delete_events, read, read_events, redo,
        remove_item, reorder_items, undo, update_item,
    },
    models::{
        errors::APIError,
        requests::CartItemUpdateRequest,
        responses::{CartItem, EventResponse, StreamResponse},
    },
    state::AppState,
};

pub fn get_cart_router() -> Router<AppState> {
    Router::new()
        .route("/", get(read_endpoint))
        .route("/undo", post(undo_endpoint))
        .route("/redo", post(redo_endpoint))
        .route("/reorder", post(reorder_items_endpoint))
        .route("/events", get(read_events_endpoint))
        .route("/events", delete(delete_events_endpoint))
        .route("/recipe/{recipe_id}", post(add_recipe_endpoint))
        .route("/item/{item_id}", post(add_item_endpoint))
        .route("/item/{item_id}", put(update_item_endpoint))
        .route("/item/{item_id}", delete(remove_item_endpoint))
        .route("/stream", get(stream_events_endpoint))
}

async fn undo_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    let cart_items = undo(&mut *queries).await?;
    state.tx.send(StreamResponse::new(cart_items.clone()).into())?;
    Ok(Json(cart_items))
}

async fn redo_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    let cart_items = redo(&mut *queries).await?;
    state.tx.send(StreamResponse::new(cart_items.clone()).into())?;
    Ok(Json(cart_items))
}

async fn add_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    let cart_items = add_item(&mut *queries, item_id).await?;
    state.tx.send(StreamResponse::new(cart_items.clone()).into())?;
    Ok(Json(cart_items))
}

async fn remove_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    let cart_items = remove_item(&mut *queries, item_id).await?;
    state.tx.send(StreamResponse::new(cart_items.clone()).into())?;
    Ok(Json(cart_items))
}

async fn update_item_endpoint(
    State(state): State<AppState>,
    Path(item_id): Path<i32>,
    Json(request): Json<CartItemUpdateRequest>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    let cart_items = update_item(&mut *queries, item_id, request).await?;
    state.tx.send(StreamResponse::new(cart_items.clone()).into())?;
    Ok(Json(cart_items))
}

async fn add_recipe_endpoint(
    State(state): State<AppState>,
    Path(recipe_id): Path<i32>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    let cart_items = add_recipe(&mut *queries, recipe_id).await?;
    state.tx.send(StreamResponse::new(cart_items.clone()).into())?;
    Ok(Json(cart_items))
}

async fn reorder_items_endpoint(
    State(state): State<AppState>,
    Json(item_ids): Json<Vec<i32>>,
) -> Result<Json<Vec<CartItem>>, APIError> {
    let mut queries = state.queries.lock().await;
    let cart_items = reorder_items(&mut *queries, item_ids).await?;
    state.tx.send(StreamResponse::new(cart_items.clone()).into())?;
    Ok(Json(cart_items))
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

async fn delete_events_endpoint(
    State(state): State<AppState>,
) -> Result<Json<Vec<EventResponse>>, APIError> {
    let mut queries = state.queries.lock().await;
    state.tx.send(StreamResponse::new(vec![]).into())?;
    Ok(Json(delete_events(&mut *queries).await?))
}

async fn stream_events_endpoint(
    State(state): State<AppState>,
) -> Sse<impl Stream<Item = Result<sse::Event, BroadcastStreamRecvError>>> {
    Sse::new(BroadcastStream::new(state.tx.subscribe()))
}
