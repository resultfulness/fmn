use std::collections::HashMap;

use axum::{Json, extract::Query};

use crate::{
    methods::echo::get_root,
    models::{
        errors::APIError, requests::EchoRequest, responses::EchoResponse,
    },
};

pub async fn get_root_endpoint(
    Query(params): Query<HashMap<String, String>>,
) -> Result<Json<EchoResponse>, APIError> {
    Ok(Json(
        get_root(EchoRequest {
            message: params
                .get("message")
                .ok_or(APIError::ValidationError)?
                .into(),
        })
        .await?,
    ))
}
