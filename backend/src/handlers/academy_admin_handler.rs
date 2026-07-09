use axum::{
    extract::{Path, State},
    Json,
};

use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::dto::{
        course::{
            CourseResponseDto,
            CreateCourseRequest,
            UpdateCourseRequest,
        },
        section::{
            SectionResponseDto,
            CreateSectionRequest,
            UpdateSectionRequest,
        },
        task::{
            TaskResponseDto,
            CreateTaskRequest,
            UpdateTaskRequest,
        },
    },
    services::academy_service,
};


// =======================
// Courses
// =======================

pub async fn create_course_handler(
    State(pool): State<PgPool>,
    Json(req): Json<CreateCourseRequest>,
) -> Result<Json<CourseResponseDto>, AppError> {

    let course = academy_service::create_course(&pool, req)
        .await?;

    Ok(Json(CourseResponseDto::from(course)))
}

pub async fn update_course_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateCourseRequest>,
) -> Result<Json<CourseResponseDto>, AppError> {

    let course = academy_service::update_course(&pool, id, req)
        .await?;

    Ok(Json(CourseResponseDto::from(course)))
}

pub async fn delete_course_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {

    academy_service::delete_course(&pool, id)
        .await?;

    Ok(())
}

// =======================
// Sections
// =======================

pub async fn get_section_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<SectionResponseDto>, AppError> {

    let section = academy_service::get_section_by_id(
        &pool,
        id,
    )
    .await?;

    Ok(Json(
        SectionResponseDto::from(section)
    ))
}

pub async fn get_sections_by_course_handler(
    State(pool): State<PgPool>,
    Path(course_id): Path<Uuid>,
) -> Result<Json<Vec<SectionResponseDto>>, AppError> {

    let sections = academy_service::get_sections_by_course(
        &pool,
        course_id,
    )
    .await?;


    let response = sections
        .into_iter()
        .map(SectionResponseDto::from)
        .collect();


    Ok(Json(response))
}

pub async fn create_section_handler(
    State(pool): State<PgPool>,
    Json(req): Json<CreateSectionRequest>,
) -> Result<Json<SectionResponseDto>, AppError> {

    let section = academy_service::create_section(&pool, req)
        .await?;

    Ok(Json(SectionResponseDto::from(section)))
}

pub async fn update_section_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateSectionRequest>,
) -> Result<Json<SectionResponseDto>, AppError> {

    let section = academy_service::update_section(&pool, id, req)
        .await?;

    Ok(Json(SectionResponseDto::from(section)))
}

pub async fn delete_section_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {

    academy_service::delete_section(&pool, id)
        .await?;

    Ok(())
}

// =======================
// Tasks
// =======================

pub async fn get_task_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<TaskResponseDto>, AppError> {

    let task = academy_service::get_task_by_id(
        &pool,
        id,
    )
    .await?;


    Ok(Json(
        TaskResponseDto::from(task)
    ))
}

pub async fn get_tasks_by_section_handler(
    State(pool): State<PgPool>,
    Path(section_id): Path<Uuid>,
) -> Result<Json<Vec<TaskResponseDto>>, AppError> {

    let tasks = academy_service::get_tasks_by_section(
        &pool,
        section_id,
    )
    .await?;


    let response = tasks
        .into_iter()
        .map(TaskResponseDto::from)
        .collect();


    Ok(Json(response))
}

pub async fn create_task_handler(
    State(pool): State<PgPool>,
    Json(req): Json<CreateTaskRequest>,
) -> Result<Json<TaskResponseDto>, AppError> {

    let task = academy_service::create_task(&pool, req)
        .await?;

    Ok(Json(TaskResponseDto::from(task)))
}

pub async fn update_task_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateTaskRequest>,
) -> Result<Json<TaskResponseDto>, AppError> {

    let task = academy_service::update_task(&pool, id, req)
        .await?;

    Ok(Json(TaskResponseDto::from(task)))
}

pub async fn delete_task_handler(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {

    academy_service::delete_task(&pool, id)
        .await?;

    Ok(())
}