use axum::{routing::{get}, Router};
use sqlx::PgPool;
use crate::handlers::{user_handler};


pub fn router() -> Router<PgPool>{
    Router::new()
    .route("/", get(user_handler::get_users))
}

