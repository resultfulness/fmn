use axum::{Json, http::StatusCode, response::IntoResponse};
use serde_json::json;

#[derive(Debug, PartialEq, Eq)]
pub enum APIError {
    InternalError,
    ValidationError,
    AlreadyExistsError,
    NotFoundError,
}
impl From<String> for APIError {
    fn from(value: String) -> Self {
        println!("{}", value);
        APIError::InternalError
    }
}
impl From<&str> for APIError {
    fn from(value: &str) -> Self {
        println!("{}", value);
        APIError::InternalError
    }
}

impl IntoResponse for APIError {
    fn into_response(self) -> axum::response::Response {
        let (status, json) = match self {
            Self::InternalError => (
                StatusCode::INTERNAL_SERVER_ERROR,
                json!({"error": "internal server error"}),
            ),
            Self::ValidationError => (
                StatusCode::UNPROCESSABLE_ENTITY,
                json!({"error": "unprocessable entity"}),
            ),
            Self::AlreadyExistsError => (
                StatusCode::CONFLICT,
                json!({"error": "entity already exists"}),
            ),
            Self::NotFoundError => (
                StatusCode::NOT_FOUND,
                json!({"error": "entity not found"}),
            ),
        };
        (status, Json(json)).into_response()
    }
}
