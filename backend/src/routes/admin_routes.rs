use axum::{
    Router, middleware,
    routing::{get, put},
};
use sqlx::PgPool;

use crate::{
    handlers::admin_handler,
    middleware::{admin_middleware::admin_middleware, auth_middleware::auth_middleware},
};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route("/dashboard", get(admin_handler::dashboard))
        .route("/users", get(admin_handler::users))
        .route("/labs", get(admin_handler::labs))
        .route("/flags", get(admin_handler::flags))
        .route(
            "/scenarios",
            get(admin_handler::scenarios).post(admin_handler::create_scenario),
        )
        .route(
            "/scenarios/:id",
            put(admin_handler::update_scenario).delete(admin_handler::delete_scenario),
        )
        .layer(middleware::from_fn(admin_middleware))
        .layer(middleware::from_fn(auth_middleware))
}
