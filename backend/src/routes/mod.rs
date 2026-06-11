pub mod user_routes;
pub mod auth;
pub mod lab_routes;

use axum::Router;
use sqlx::PgPool;


pub fn create_api_router() -> Router<PgPool> {
    Router::new()
    .nest("/auth", auth::router())    
    .nest("/users", user_routes::router())    
    .nest("/lab", lab_routes::router())
}