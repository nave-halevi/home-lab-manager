use axum::{routing::{get}, Router};
use sqlx::PgPool;
use crate::handlers::{user_handler};

use crate::middleware::auth_middleware::auth_middleware;
use crate::middleware::admin_middleware::admin_middleware;

pub fn router() -> Router<PgPool>{
    Router::new()
    .route("/", get(user_handler::get_users))
    .layer(axum::middleware::from_fn(admin_middleware))
    .layer(axum::middleware::from_fn(auth_middleware))
}

