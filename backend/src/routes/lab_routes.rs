use axum::{routing::{post, get}, Router};
use sqlx::PgPool;
use crate::handlers::{lab_handler, terminal_handler};

pub fn router() -> Router<PgPool>{
    Router::new()
        .route("/create", post(lab_handler::handle_create_lab))
        .route("/delete", post(lab_handler::handle_delete_lab))
        .route("/terminal/:port", get(terminal_handler::ws_terminal_handler))
}