use axum::{Router, middleware, routing::get};
use sqlx::PgPool;

use crate::{handlers::dashboard_handler, middleware::auth_middleware::auth_middleware};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/", get(dashboard_handler::get_dashboard_handler))
        .route_layer(middleware::from_fn(auth_middleware))
}
