use axum::{
    Router, middleware,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::{
    handlers::{lab_handler, terminal_handler},
    middleware::auth_middleware::auth_middleware,
};

pub fn router() -> Router<PgPool> {
    let protected_routes = Router::new()
        .route("/create", post(lab_handler::handle_create_lab))
        .route("/delete", post(lab_handler::handle_delete_lab))
        .route("/submit", post(lab_handler::handle_submit_flag))
        .route(
            "/status/:environment_id",
            get(lab_handler::handle_get_lab_status),
        )
        .route("/active", get(lab_handler::handle_get_any_active_lab))
        .route(
            "/active/:scenario_id",
            get(lab_handler::handle_get_active_lab),
        )
        .route_layer(middleware::from_fn(auth_middleware));

    Router::new().merge(protected_routes).route(
        "/terminal/:environment_id",
        get(terminal_handler::ws_terminal_handler),
    )
}
