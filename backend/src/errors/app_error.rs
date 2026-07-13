use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};

use serde::Serialize;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Database error")]
    Database(#[from] sqlx::Error),

    #[error("Resource not found")]
    NotFound,

    #[error("Unauthorized")]
    Unauthorized,

    #[error("Forbidden")]
    Forbidden,

    #[error("Validation error: {0}")]
    Validation(String),

    #[error("Internal server error")]
    Internal,
}

#[derive(Debug, Serialize)]
struct ErrorResponse {
    message: String,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            AppError::Database(err) => {
                tracing::error!("Database error: {:?}", err);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    "Database error".to_string(),
                )
            }

            AppError::NotFound => (StatusCode::NOT_FOUND, "Resource not found".to_string()),

            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, "Unauthorized".to_string()),

            AppError::Forbidden => (StatusCode::FORBIDDEN, "Forbidden".to_string()),

            AppError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),

            AppError::Internal => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Internal server error".to_string(),
            ),
        };

        (status, Json(ErrorResponse { message })).into_response()
    }
}
