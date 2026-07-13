use crate::models::user::{LoginRequest, RegisterRequest};
use crate::services::auth_service;
use axum::http::StatusCode;
use axum::{Json, extract::State, response::IntoResponse};
use sqlx::PgPool;

pub async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match auth_service::register(&pool, payload).await {
        Ok(user) => (StatusCode::CREATED, Json(user)).into_response(),
        Err(e) => (StatusCode::BAD_REQUEST, e).into_response(),
    }
}

pub async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    match auth_service::login(&pool, payload).await {
        Ok(response) => (StatusCode::OK, Json(response)).into_response(),

        Err(e) => (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({ "error": e })),
        )
            .into_response(),
    }
}
