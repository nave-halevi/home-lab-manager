use axum::{
    Router,
    routing::{get, post, put},
};

use crate::handlers::{academy_admin_handler, academy_handler};
use sqlx::PgPool;

use crate::middleware::{admin_middleware::admin_middleware, auth_middleware::auth_middleware};

pub fn academy_routes() -> Router<PgPool> {
    Router::new()
        // Public
        .nest("/courses", public_courses())
        // Admin
        .nest("/admin", admin_routes())
}

fn public_courses() -> Router<PgPool> {
    Router::new()
        .route("/", get(academy_handler::get_courses_handler))
        .route("/:id/full", get(academy_handler::get_course_full_handler))
}

fn admin_routes() -> Router<PgPool> {
    Router::new()
        // =======================
        // Courses
        // =======================
        .route(
            "/courses",
            post(academy_admin_handler::create_course_handler),
        )
        .route(
            "/courses/:id",
            put(academy_admin_handler::update_course_handler)
                .delete(academy_admin_handler::delete_course_handler),
        )
        // =======================
        // Sections
        // =======================
        .route(
            "/sections",
            post(academy_admin_handler::create_section_handler),
        )
        .route(
            "/sections/:id",
            get(academy_admin_handler::get_section_handler)
                .put(academy_admin_handler::update_section_handler)
                .delete(academy_admin_handler::delete_section_handler),
        )
        .route(
            "/courses/:course_id/sections",
            get(academy_admin_handler::get_sections_by_course_handler),
        )
        // =======================
        // Tasks
        // =======================
        .route("/tasks", post(academy_admin_handler::create_task_handler))
        .route(
            "/tasks/:id",
            get(academy_admin_handler::get_task_handler)
                .put(academy_admin_handler::update_task_handler)
                .delete(academy_admin_handler::delete_task_handler),
        )
        .route(
            "/sections/:section_id/tasks",
            get(academy_admin_handler::get_tasks_by_section_handler),
        )
        // =======================
        // Middleware
        // =======================
        .layer(axum::middleware::from_fn(admin_middleware))
        .layer(axum::middleware::from_fn(auth_middleware))
}
