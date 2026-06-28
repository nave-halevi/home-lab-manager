use axum::{Json, extract::State, response::IntoResponse};
use sqlx::PgPool;
use crate::models::user::{RegisterRequest, LoginRequest};
use crate::services::auth_service;
use axum::http::StatusCode;

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
        Ok(token) => (
            StatusCode::OK,
            Json(serde_json::json!({ "token": token }))).into_response(),
        Err(e) => (StatusCode::UNAUTHORIZED, e).into_response(),
    }
}