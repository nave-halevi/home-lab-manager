use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::dto::{
        dashboard::{
            AvailableCourseDto, CompletedCourseDto, DashboardCourseDto, DashboardCurrentTaskDto,
            DashboardResponseDto, DashboardStatisticsDto, DashboardUserDto,
        },
        task_progress::TaskProgressStatus,
    },
    repositories::dashboard_repo,
    services::task_progress_service,
};

pub async fn get_dashboard(pool: &PgPool, user_id: Uuid) -> Result<DashboardResponseDto, AppError> {
    let user = dashboard_repo::get_dashboard_user(pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    let started_course_ids = dashboard_repo::get_started_course_ids(pool, user_id).await?;

    let available_course_rows = dashboard_repo::get_available_courses(pool, user_id).await?;

    let mut continue_learning = Vec::new();
    let mut completed_course_list = Vec::new();

    let mut completed_courses = 0_i64;
    let mut completed_tasks = 0_i64;

    for course_id in started_course_ids {
        let progress = task_progress_service::get_course_progress(pool, user_id, course_id).await?;

        let course_details = dashboard_repo::get_course_details(pool, course_id)
            .await?
            .ok_or(AppError::NotFound)?;

        completed_tasks += progress.completed_tasks;

        let course_completed =
            progress.total_tasks > 0 && progress.completed_tasks == progress.total_tasks;

        if course_completed {
            completed_courses += 1;

            completed_course_list.push(CompletedCourseDto {
                course_id: progress.course_id,
                course_title: progress.course_title,
                course_description: course_details.course_description,
                difficulty: course_details.difficulty,

                completed_tasks: progress.completed_tasks,
                total_tasks: progress.total_tasks,
                progress_percentage: progress.progress_percentage,

                earned_points: progress.earned_points,
                total_points: progress.total_points,
            });

            continue;
        }

        let current_task = progress
            .tasks
            .iter()
            .find(|task| task.progress_status == TaskProgressStatus::InProgress)
            .or_else(|| {
                progress.tasks.iter().find(|task| {
                    task.access_status
                        == crate::models::dto::task_progress::TaskAccessStatus::Available
                        && task.progress_status != TaskProgressStatus::Completed
                })
            })
            .map(|task| DashboardCurrentTaskDto {
                task_id: task.task_id,
                task_title: task.task_title.clone(),
                task_type: task.task_type.clone(),
            });

        continue_learning.push(DashboardCourseDto {
            course_id: progress.course_id,
            course_title: progress.course_title,
            course_description: course_details.course_description,
            difficulty: course_details.difficulty,

            completed_tasks: progress.completed_tasks,
            total_tasks: progress.total_tasks,
            progress_percentage: progress.progress_percentage,

            earned_points: progress.earned_points,
            total_points: progress.total_points,

            current_task,
        });
    }

    let active_courses = continue_learning.len() as i64;

    let available_courses = available_course_rows
        .into_iter()
        .map(|course| AvailableCourseDto {
            course_id: course.course_id,
            course_title: course.course_title,
            course_description: course.course_description,
            difficulty: course.difficulty,
            total_tasks: course.total_tasks,
            total_points: course.total_points,
        })
        .collect();

    Ok(DashboardResponseDto {
        user: DashboardUserDto {
            user_name: user.user_name,
            total_score: user.total_score,
        },

        statistics: DashboardStatisticsDto {
            active_courses,
            completed_courses,
            completed_tasks,
        },

        continue_learning,
        completed_courses: completed_course_list,
        available_courses,
    })
}
