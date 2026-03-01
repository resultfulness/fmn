use std::collections::HashMap;

use axum::{
    Json,
    extract::{Query, State},
};

use crate::{
    methods::echo::get_root,
    models::{
        errors::APIError, requests::EchoRequest, responses::EchoResponse,
    },
    state::AppState,
};

pub async fn get_root_endpoint(
    State(state): State<AppState>,
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<EchoResponse>, APIError> {
    let mut queries = state.queries.lock().await;
    Ok(Json(
        get_root(
            &mut *queries,
            EchoRequest {
                message: params
                    .get("message")
                    .ok_or(APIError::ValidationError)?
                    .into(),
            },
        )
        .await?,
    ))
}
