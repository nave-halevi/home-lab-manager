use sqlx::PgPool;
use std::collections::HashMap;
use uuid::Uuid;

use crate::errors::AppError;

use crate::repositories::academy_repo;

use crate::models::entities::{course::Course, section::Section, task::Task};

use crate::models::dto::{
    course::{CourseFullDto, CreateCourseRequest, UpdateCourseRequest},
    section::{CreateSectionRequest, SectionDto, UpdateSectionRequest},
    task::{CreateTaskRequest, TaskDto, UpdateTaskRequest},
};

// =====================================================
// Courses
// =====================================================

pub async fn get_courses(pool: &PgPool) -> Result<Vec<Course>, AppError> {
    Ok(academy_repo::get_courses(pool).await?)
}

pub async fn get_course_by_id(pool: &PgPool, id: Uuid) -> Result<Course, AppError> {
    academy_repo::get_course_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)
}

pub async fn get_course_full(pool: &PgPool, course_id: Uuid) -> Result<CourseFullDto, AppError> {
    let course = get_course_by_id(pool, course_id).await?;

    let rows = academy_repo::get_course_full_rows(pool, course_id).await?;

    let mut sections: HashMap<Uuid, SectionDto> = HashMap::new();

    for row in rows {
        let section = sections.entry(row.section_id).or_insert(SectionDto {
            id: row.section_id,
            title: row.section_title,
            order_index: row.section_order,
            tasks: vec![],
        });

        if let Some(task_id) = row.task_id {
            section.tasks.push(TaskDto {
                id: task_id,
                section_id: row.task_section_id.unwrap(),
                scenario_id: row.scenario_id,
                title: row.task_title.unwrap(),
                content: row.task_content.unwrap(),
                task_type: row.task_type.unwrap(),
                order_index: row.task_order.unwrap(),
                points: row.points.unwrap(),
            });
        }
    }

    let mut sections: Vec<SectionDto> = sections.into_values().collect();

    sections.sort_by_key(|s| s.order_index);

    Ok(CourseFullDto {
        id: course.id,

        title: course.title,

        slug: course.slug,

        description: course.description,

        difficulty: course.difficulty,

        is_published: course.is_published,

        created_at: course.created_at,

        sections,
    })
}

pub async fn create_course(pool: &PgPool, req: CreateCourseRequest) -> Result<Course, AppError> {
    let course = academy_repo::create_course(
        pool,
        &req.title,
        &req.slug,
        req.description.as_deref(),
        req.difficulty.as_deref(),
    )
    .await?;

    Ok(course)
}

pub async fn update_course(
    pool: &PgPool,
    id: Uuid,
    req: UpdateCourseRequest,
) -> Result<Course, AppError> {
    let course = academy_repo::update_course(pool, id, req).await?;

    Ok(course)
}

pub async fn delete_course(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    academy_repo::delete_course(pool, id).await?;

    Ok(())
}

// =====================================================
// Sections
// =====================================================

pub async fn get_section_by_id(pool: &PgPool, id: Uuid) -> Result<Section, AppError> {
    academy_repo::get_section_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)
}

pub async fn get_sections_by_course(
    pool: &PgPool,
    course_id: Uuid,
) -> Result<Vec<Section>, AppError> {
    Ok(academy_repo::get_sections_by_course(pool, course_id).await?)
}

pub async fn create_section(pool: &PgPool, req: CreateSectionRequest) -> Result<Section, AppError> {
    let section = academy_repo::create_section(
        pool,
        req.course_id,
        &req.title,
        req.description.as_deref(),
        req.order_index,
    )
    .await?;

    Ok(section)
}

pub async fn update_section(
    pool: &PgPool,
    id: Uuid,
    req: UpdateSectionRequest,
) -> Result<Section, AppError> {
    let section = academy_repo::update_section(pool, id, req).await?;

    Ok(section)
}

pub async fn delete_section(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    academy_repo::delete_section(pool, id).await?;

    Ok(())
}

// =====================================================
// Tasks
// =====================================================

pub async fn get_task_by_id(pool: &PgPool, id: Uuid) -> Result<Task, AppError> {
    academy_repo::get_task_by_id(pool, id)
        .await?
        .ok_or(AppError::NotFound)
}

pub async fn get_tasks_by_section(pool: &PgPool, section_id: Uuid) -> Result<Vec<Task>, AppError> {
    Ok(academy_repo::get_tasks_by_section(pool, section_id).await?)
}

pub async fn create_task(pool: &PgPool, req: CreateTaskRequest) -> Result<Task, AppError> {
    Ok(academy_repo::create_task(pool, req).await?)
}

pub async fn update_task(
    pool: &PgPool,
    id: Uuid,
    req: UpdateTaskRequest,
) -> Result<Task, AppError> {
    Ok(academy_repo::update_task(pool, id, req).await?)
}

pub async fn delete_task(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    academy_repo::delete_task(pool, id).await?;

    Ok(())
}
