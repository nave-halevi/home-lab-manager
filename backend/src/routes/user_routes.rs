use axum::{routing::{post, get}, Router};
use sqlx::PgPool;
use crate::handlers::{chat_handler, user_handler};


pub async fn build_user_routes() -> Router<PgPool>{
    Router::new()
    .route("/register", post(user_handler::register_user))
    .route("/", get(user_handler::get_users))
    .route("/login", post(user_handler::login_user))
    .route("/ws", get(chat_handler::ws_handler))    
}

