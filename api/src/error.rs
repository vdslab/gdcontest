use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
};

pub enum ApiError {
    Forbidden(String),
    NotFound(String),
    Unknown(String),
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        match self {
            ApiError::Forbidden(message) => (StatusCode::UNAUTHORIZED, message).into_response(),
            ApiError::NotFound(message) => (StatusCode::NOT_FOUND, message).into_response(),
            ApiError::Unknown(message) => {
                (StatusCode::INTERNAL_SERVER_ERROR, message).into_response()
            }
        }
    }
}

pub type Result<T> = std::result::Result<T, ApiError>;
