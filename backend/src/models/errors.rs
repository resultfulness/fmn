use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

pub enum APIError {
    IDontLikeErrorWord,
    ValidationError,
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let (status, json) = match self {
            Self::IDontLikeErrorWord => (
                StatusCode::BAD_REQUEST,
                json!({"error": "i dont like the error word"}),
            ),
            Self::ValidationError => (
                StatusCode::UNPROCESSABLE_ENTITY,
                json!({"error": "unprocessable entity"}),
            ),
        };
        (status, Json(json)).into_response()
    }
}
