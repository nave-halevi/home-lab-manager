use axum::{routing::{post}, Router};
use sqlx::PgPool;
use crate::handlers::lab_handler;

pub fn router() -> Router<PgPool>{
    Router::new()
        .route("/create", post(lab_handler::handle_create_lab))
        .route("/delete", post(lab_handler::handle_delete_lab))
}