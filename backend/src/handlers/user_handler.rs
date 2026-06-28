use axum::{extract::State, Json, http::StatusCode, response::IntoResponse};
use sqlx::PgPool;
use crate::services::user_service;


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

