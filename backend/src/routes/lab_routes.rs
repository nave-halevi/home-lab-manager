use axum::{
    routing::{
        get,
        post,
    },
    Router,
};
use sqlx::PgPool;

use crate::handlers::{
    lab_handler,
    terminal_handler,
};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/create",
            post(lab_handler::handle_create_lab),
        )
        .route(
            "/delete",
            post(lab_handler::handle_delete_lab),
        )
        .route(
            "/submit",
            post(lab_handler::handle_submit_flag),
        )
        .route(
            "/status/:user_id/:environment_id",
            get(lab_handler::handle_get_lab_status),
        )
        .route(
            "/active/:user_id/:scenario_id",
            get(lab_handler::handle_get_active_lab),
        )
        .route(
            "/terminal/:environment_id",
            get(terminal_handler::ws_terminal_handler),
        )
}