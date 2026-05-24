use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use crate::models::user::{RegisterRequest, LoginRequest};
use crate::services::user_service;

pub async fn register_user(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> impl IntoResponse {
    match user_service::register_new_user(&pool, payload).await {
        Ok(user) => {
            (StatusCode::CREATED, Json(user)).into_response()
        }

        Err(e) => {
            (StatusCode::BAD_REQUEST, e.to_string()).into_response()
        }
    }
}

pub async fn get_users(State(pool): State<PgPool>) -> impl IntoResponse {
    match user_service::fetch_all_users(&pool).await {

        Ok(users) => {
            (StatusCode::OK, Json(users)).into_response()
        }
        Err(e) => {
            (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()).into_response()
        }
    }
}

pub async fn login_user(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> impl IntoResponse {
    
    match user_service::login_user(&pool, payload).await {
        
        Ok(token) => {
            (StatusCode::OK, Json(serde_json::json!({"token": token}))).into_response()
        }
        Err(e) => {
            (StatusCode::UNAUTHORIZED, e).into_response()
        }
    }
}