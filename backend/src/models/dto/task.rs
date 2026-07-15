use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::entities::task::Task;

#[derive(Debug, Serialize)]
pub struct TaskDto {
    pub id: Uuid,
    pub section_id: Uuid,
    pub scenario_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub task_type: String,
    pub order_index: i32,
    pub points: i32,
}

#[derive(Debug, Serialize)]
pub struct TaskResponseDto {
    pub id: Uuid,
    pub section_id: Uuid,
    pub scenario_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub task_type: String,
    pub order_index: i32,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateTaskRequest {
    pub section_id: Uuid,
    pub scenario_id: Option<Uuid>,
    pub title: String,
    pub content: String,
    pub task_type: String,
    pub order_index: i32,
    pub points: Option<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaskRequest {
    pub title: Option<String>,
    pub content: Option<String>,
    pub task_type: Option<String>,
    pub scenario_id: Option<Option<Uuid>>,
    pub order_index: Option<i32>,
    pub points: Option<i32>,
}

impl From<Task> for TaskResponseDto {
    fn from(task: Task) -> Self {
        Self {
            id: task.id,
            section_id: task.section_id,
            scenario_id: task.scenario_id,
            title: task.title,
            content: task.content,
            task_type: task.task_type,
            order_index: task.order_index,
            points: task.points,
        }
    }
}
