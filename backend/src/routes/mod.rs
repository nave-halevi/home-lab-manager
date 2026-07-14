pub mod academy_routes;
pub mod auth;
pub mod dashboard_routes;
pub mod lab_routes;
pub mod task_progress_routes;
pub mod user_routes;

use axum::Router;
use sqlx::PgPool;

pub fn create_api_router() -> Router<PgPool> {
    Router::new()
        .nest("/auth", auth::router())
        .nest("/users", user_routes::router())
        .nest("/lab", lab_routes::router())
        .nest("/academy", academy_routes::academy_routes())
        .nest("/task-progress", task_progress_routes::router())
        .nest("/dashboard", dashboard_routes::router())
}
