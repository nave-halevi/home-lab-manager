use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct DashboardUserDto {
    pub user_name: String,
    pub total_score: i32,
}

#[derive(Debug, Serialize)]
pub struct DashboardStatisticsDto {
    pub active_courses: i64,
    pub completed_courses: i64,
    pub completed_tasks: i64,
}

#[derive(Debug, Serialize)]
pub struct DashboardCurrentTaskDto {
    pub task_id: Uuid,
    pub task_title: String,
    pub task_type: String,
}

#[derive(Debug, Serialize)]
pub struct DashboardCourseDto {
    pub course_id: Uuid,
    pub course_title: String,
    pub course_description: Option<String>,
    pub difficulty: Option<String>,

    pub completed_tasks: i64,
    pub total_tasks: i64,
    pub progress_percentage: f64,

    pub earned_points: i64,
    pub total_points: i64,

    pub current_task: Option<DashboardCurrentTaskDto>,
}

#[derive(Debug, Serialize)]
pub struct AvailableCourseDto {
    pub course_id: Uuid,
    pub course_title: String,
    pub course_description: Option<String>,
    pub difficulty: Option<String>,
    pub total_tasks: i64,
    pub total_points: i64,
}

#[derive(Debug, Serialize)]
pub struct DashboardResponseDto {
    pub user: DashboardUserDto,
    pub statistics: DashboardStatisticsDto,

    pub continue_learning: Vec<DashboardCourseDto>,
    pub available_courses: Vec<AvailableCourseDto>,
    pub completed_courses: Vec<CompletedCourseDto>,
}

#[derive(Debug, Serialize)]
pub struct CompletedCourseDto {
    pub course_id: Uuid,
    pub course_title: String,
    pub course_description: Option<String>,
    pub difficulty: Option<String>,

    pub completed_tasks: i64,
    pub total_tasks: i64,
    pub progress_percentage: f64,

    pub earned_points: i64,
    pub total_points: i64,
}
