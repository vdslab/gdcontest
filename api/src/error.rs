use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

#[derive(Clone)]
pub enum ApiError {
    Forbidden(String),
    NotFound(String),
    Unknown(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        let (status_code, message) = match self {
            ApiError::Forbidden(message) => (StatusCode::UNAUTHORIZED, message),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message),
            ApiError::Unknown(message) => (StatusCode::INTERNAL_SERVER_ERROR, message),
        };
        (status_code, Json(ErrorContent { message })).into_response()
    }
}

#[derive(Serialize)]
pub struct ErrorContent {
    pub message: String,
}

pub type Result<T> = std::result::Result<T, ApiError>;
