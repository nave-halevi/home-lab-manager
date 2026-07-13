use chrono::{DateTime, Utc};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskProgressStatus {
    NotStarted,
    InProgress,
    Completed,
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum TaskAccessStatus {
    Available,
    Locked,
}

#[derive(Debug, Serialize)]
pub struct TaskProgressDto {
    pub task_id: Uuid,
    pub task_title: String,
    pub task_type: String,
    pub task_order_index: i32,

    pub section_id: Uuid,
    pub section_title: String,
    pub section_order_index: i32,

    pub points: i32,
    pub progress_status: TaskProgressStatus,
    pub access_status: TaskAccessStatus,

    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct CourseProgressDto {
    pub course_id: Uuid,
    pub course_title: String,

    pub total_tasks: i64,
    pub completed_tasks: i64,
    pub progress_percentage: f64,

    pub total_points: i64,
    pub earned_points: i64,

    pub tasks: Vec<TaskProgressDto>,
}
