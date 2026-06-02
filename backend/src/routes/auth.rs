use axum::{routing::{post}, Router};
use sqlx::PgPool;
use crate::handlers::{user_handler};

pub fn router() -> Router<PgPool> {
Router::new()
    .route("/register", post(user_handler::register_user))
    .route("/login", post(user_handler::login_user))
}