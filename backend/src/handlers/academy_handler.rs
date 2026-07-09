use axum::{
    extract::{Path, State},
    Json,
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,

    models::{
        dto::{
            course::CourseResponseDto,
            course::CourseFullDto,
        },
    },

    services::academy_service,
};

pub async fn get_courses_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CourseResponseDto>>, AppError> {


    let courses = academy_service::get_courses(&pool)
        .await?;


    let response = courses
        .into_iter()
        .map(CourseResponseDto::from)
        .collect();


    Ok(Json(response))
}

pub async fn get_course_full_handler(
    State(pool): State<PgPool>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<CourseFullDto>, AppError> {


    let course = academy_service::get_course_full(
        &pool,
        course_id,
    )
    .await?;


    Ok(Json(course))
}