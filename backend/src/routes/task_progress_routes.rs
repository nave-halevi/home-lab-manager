use axum::{
    Router, middleware,
    routing::{get, post},
};
use sqlx::PgPool;

use crate::{handlers::task_progress_handler, middleware::auth_middleware::auth_middleware};

pub fn router() -> Router<PgPool> {
    Router::new()
        .route(
            "/courses/:course_id",
            get(task_progress_handler::get_course_progress_handler),
        )
        .route(
            "/tasks/:task_id/complete",
            post(task_progress_handler::complete_content_task_handler),
        )
        .route(
            "/tasks/:task_id/start",
            post(task_progress_handler::start_task_handler),
        )
        .route_layer(middleware::from_fn(auth_middleware))
}
