use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::dto::task_progress::{
        CourseProgressDto, TaskAccessStatus, TaskProgressDto, TaskProgressStatus,
    },
    repositories::{academy_repo, task_progress_repo},
};

pub async fn ensure_task_accessible(
    pool: &PgPool,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<(), AppError> {
    let course_id = task_progress_repo::get_course_id_by_task_id(pool, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let course_progress = get_course_progress(pool, user_id, course_id).await?;

    let task_progress = course_progress
        .tasks
        .iter()
        .find(|task| task.task_id == task_id)
        .ok_or(AppError::NotFound)?;

    match task_progress.access_status {
        TaskAccessStatus::Available => Ok(()),

        TaskAccessStatus::Locked => Err(AppError::Forbidden),
    }
}

pub async fn complete_content_task(
    pool: &PgPool,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<CourseProgressDto, AppError> {
    let course_id = task_progress_repo::get_course_id_by_task_id(pool, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let current_progress = get_course_progress(pool, user_id, course_id).await?;

    let task = current_progress
        .tasks
        .iter()
        .find(|task| task.task_id == task_id)
        .ok_or(AppError::NotFound)?;

    if task.task_type == "LAB" {
        return Err(AppError::Validation(
            "Lab tasks can only be completed by submitting the correct flag.".to_string(),
        ));
    }

    ensure_task_accessible(pool, user_id, task_id).await?;

    task_progress_repo::mark_task_completed(pool, user_id, task_id).await?;

    get_course_progress(pool, user_id, course_id).await
}

fn apply_access_status(tasks: &mut [TaskProgressDto]) {
    let mut found_first_incomplete = false;

    for task in tasks {
        match task.progress_status {
            TaskProgressStatus::Completed => {
                task.access_status = TaskAccessStatus::Available;
            }

            TaskProgressStatus::NotStarted | TaskProgressStatus::InProgress => {
                if found_first_incomplete {
                    task.access_status = TaskAccessStatus::Locked;
                } else {
                    task.access_status = TaskAccessStatus::Available;
                    found_first_incomplete = true;
                }
            }
        }
    }
}

pub async fn get_course_progress(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
) -> Result<CourseProgressDto, AppError> {
    let course = academy_repo::get_course_by_id(pool, course_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let rows = task_progress_repo::get_course_progress(pool, user_id, course_id).await?;

    let total_tasks = rows.len() as i64;

    let completed_tasks = rows.iter().filter(|row| row.status == "COMPLETED").count() as i64;

    let total_points = rows.iter().map(|row| i64::from(row.points)).sum();

    let earned_points = rows
        .iter()
        .filter(|row| row.status == "COMPLETED")
        .map(|row| i64::from(row.points))
        .sum();

    let progress_percentage = if total_tasks == 0 {
        0.0
    } else {
        let percentage = completed_tasks as f64 / total_tasks as f64 * 100.0;

        (percentage * 100.0).round() / 100.0
    };

    let mut tasks: Vec<TaskProgressDto> = rows
        .into_iter()
        .map(|row| TaskProgressDto {
            task_id: row.task_id,
            task_title: row.task_title,
            task_type: row.task_type,
            task_order_index: row.task_order_index,

            section_id: row.section_id,
            section_title: row.section_title,
            section_order_index: row.section_order_index,

            points: row.points,

            progress_status: match row.status.as_str() {
                "COMPLETED" => TaskProgressStatus::Completed,
                "IN_PROGRESS" => TaskProgressStatus::InProgress,
                _ => TaskProgressStatus::NotStarted,
            },

            access_status: TaskAccessStatus::Available,

            started_at: row.started_at,
            completed_at: row.completed_at,
        })
        .collect();

    apply_access_status(&mut tasks);

    Ok(CourseProgressDto {
        course_id: course.id,
        course_title: course.title,

        total_tasks,
        completed_tasks,
        progress_percentage,

        total_points,
        earned_points,

        tasks,
    })
}

pub async fn start_task(
    pool: &PgPool,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<CourseProgressDto, AppError> {
    let course_id = task_progress_repo::get_course_id_by_task_id(pool, task_id)
        .await?
        .ok_or(AppError::NotFound)?;

    ensure_task_accessible(pool, user_id, task_id).await?;

    task_progress_repo::mark_task_in_progress(pool, user_id, task_id).await?;

    get_course_progress(pool, user_id, course_id).await
}
