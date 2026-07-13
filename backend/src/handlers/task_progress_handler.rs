use axum::{
    Json,
    extract::{Extension, Path, State},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{dto::task_progress::CourseProgressDto, user::Claims},
    services::task_progress_service,
};

pub async fn get_course_progress_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<CourseProgressDto>, AppError> {
    /*
        The user ID comes only from the verified JWT claims.

        We deliberately do not accept a user ID from the URL,
        query parameters, or request body.
    */
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

    let progress = task_progress_service::get_course_progress(&pool, user_id, course_id).await?;

    Ok(Json(progress))
}

pub async fn complete_content_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(task_id): Path<Uuid>,
) -> Result<Json<CourseProgressDto>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

    let progress = task_progress_service::complete_content_task(&pool, user_id, task_id).await?;

    Ok(Json(progress))
}
