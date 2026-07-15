use axum::{
    Router, middleware,
    routing::{get, post, put},
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
        .route("/users/:user_id", get(admin_handler::user_details))
        .route(
            "/users/:user_id/status",
            put(admin_handler::update_user_status),
        )
        .route("/users/:user_id/role", put(admin_handler::update_user_role))
        .route(
            "/users/:user_id/password",
            put(admin_handler::reset_user_password),
        )
        .route("/labs", get(admin_handler::labs))
        .route(
            "/labs/:environment_id/terminate",
            post(admin_handler::terminate_lab),
        )
        .route(
            "/flags",
            get(admin_handler::flags).post(admin_handler::create_flag),
        )
        .route(
            "/flags/:id",
            get(admin_handler::flag)
                .put(admin_handler::update_flag)
                .delete(admin_handler::delete_flag),
        )
        .route("/activity", get(admin_handler::activity))
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
