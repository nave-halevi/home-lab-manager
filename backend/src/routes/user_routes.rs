use crate::handlers::user_handler;
use axum::{Router, routing::get};
use sqlx::PgPool;

use crate::middleware::admin_middleware::admin_middleware;
use crate::middleware::auth_middleware::auth_middleware;

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(user_handler::get_users))
        .layer(axum::middleware::from_fn(admin_middleware))
        .layer(axum::middleware::from_fn(auth_middleware))
}
