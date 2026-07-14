use axum::{
    Router,
    extract::DefaultBodyLimit,
    middleware,
    routing::{get, put},
};
use sqlx::PgPool;

use crate::{handlers::profile_handler, middleware::auth_middleware::auth_middleware};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/",
            get(profile_handler::get_profile_handler).put(profile_handler::update_profile_handler),
        )
        .route("/password", put(profile_handler::change_password_handler))
        .route("/avatar", put(profile_handler::update_avatar_handler))
        .route_layer(middleware::from_fn(auth_middleware))
        .layer(DefaultBodyLimit::max(3 * 1024 * 1024))
}
