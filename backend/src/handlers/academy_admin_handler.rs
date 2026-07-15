use axum::{
    Extension, Json,
    extract::{Path, State},
};

use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::dto::{
        course::{CourseResponseDto, CreateCourseRequest, UpdateCourseRequest},
        section::{CreateSectionRequest, SectionResponseDto, UpdateSectionRequest},
        task::{CreateTaskRequest, TaskResponseDto, UpdateTaskRequest},
    },
    models::user::Claims,
    services::{academy_service, admin_service},
};

fn admin_id(claims: &Claims) -> Result<Uuid, AppError> {
    claims.sub.parse().map_err(|_| AppError::Unauthorized)
}

// =======================
// Courses
// =======================

pub async fn get_courses_handler(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<CourseResponseDto>>, AppError> {
    let courses = academy_service::get_all_courses(&pool).await?;
    Ok(Json(
        courses.into_iter().map(CourseResponseDto::from).collect(),
    ))
}

pub async fn create_course_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateCourseRequest>,
) -> Result<Json<CourseResponseDto>, AppError> {
    let course = academy_service::create_course(&pool, req).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Course created",
        "Course",
        Some(course.id),
        Some(json!({ "title": course.title, "slug": course.slug })),
    )
    .await;

    Ok(Json(CourseResponseDto::from(course)))
}

pub async fn update_course_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCourseRequest>,
) -> Result<Json<CourseResponseDto>, AppError> {
    let course = academy_service::update_course(&pool, id, req).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Course updated",
        "Course",
        Some(course.id),
        Some(json!({ "title": course.title, "slug": course.slug })),
    )
    .await;

    Ok(Json(CourseResponseDto::from(course)))
}

pub async fn delete_course_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    academy_service::delete_course(&pool, id).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Course deleted",
        "Course",
        Some(id),
        None,
    )
    .await;

    Ok(())
}

// =======================
// Sections
// =======================

pub async fn get_section_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SectionResponseDto>, AppError> {
    let section = academy_service::get_section_by_id(&pool, id).await?;

    Ok(Json(SectionResponseDto::from(section)))
}

pub async fn get_sections_by_course_handler(
    State(pool): State<PgPool>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<Vec<SectionResponseDto>>, AppError> {
    let sections = academy_service::get_sections_by_course(&pool, course_id).await?;

    let response = sections.into_iter().map(SectionResponseDto::from).collect();

    Ok(Json(response))
}

pub async fn create_section_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateSectionRequest>,
) -> Result<Json<SectionResponseDto>, AppError> {
    let section = academy_service::create_section(&pool, req).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Section created",
        "Section",
        Some(section.id),
        Some(json!({ "course_id": section.course_id, "title": section.title })),
    )
    .await;

    Ok(Json(SectionResponseDto::from(section)))
}

pub async fn update_section_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSectionRequest>,
) -> Result<Json<SectionResponseDto>, AppError> {
    let section = academy_service::update_section(&pool, id, req).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Section updated",
        "Section",
        Some(section.id),
        Some(json!({ "course_id": section.course_id, "title": section.title })),
    )
    .await;

    Ok(Json(SectionResponseDto::from(section)))
}

pub async fn delete_section_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    academy_service::delete_section(&pool, id).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Section deleted",
        "Section",
        Some(id),
        None,
    )
    .await;

    Ok(())
}

// =======================
// Tasks
// =======================

pub async fn get_task_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskResponseDto>, AppError> {
    let task = academy_service::get_task_by_id(&pool, id).await?;

    Ok(Json(TaskResponseDto::from(task)))
}

pub async fn get_tasks_by_section_handler(
    State(pool): State<PgPool>,
    Path(section_id): Path<Uuid>,
) -> Result<Json<Vec<TaskResponseDto>>, AppError> {
    let tasks = academy_service::get_tasks_by_section(&pool, section_id).await?;

    let response = tasks.into_iter().map(TaskResponseDto::from).collect();

    Ok(Json(response))
}

pub async fn create_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponseDto>, AppError> {
    let task = academy_service::create_task(&pool, req).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Task created",
        "Task",
        Some(task.id),
        Some(json!({
            "section_id": task.section_id,
            "title": task.title,
            "task_type": task.task_type,
        })),
    )
    .await;

    Ok(Json(TaskResponseDto::from(task)))
}

pub async fn update_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponseDto>, AppError> {
    let task = academy_service::update_task(&pool, id, req).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Task updated",
        "Task",
        Some(task.id),
        Some(json!({
            "section_id": task.section_id,
            "title": task.title,
            "task_type": task.task_type,
        })),
    )
    .await;

    Ok(Json(TaskResponseDto::from(task)))
}

pub async fn delete_task_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    academy_service::delete_task(&pool, id).await?;
    admin_service::record_activity(
        &pool,
        admin_id(&claims)?,
        "Task deleted",
        "Task",
        Some(id),
        None,
    )
    .await;

    Ok(())
}
