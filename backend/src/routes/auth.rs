use crate::handlers::auth_handler;
use axum::{Router, routing::post};
use sqlx::PgPool;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/register", post(auth_handler::register))
        .route("/login", post(auth_handler::login))
}
