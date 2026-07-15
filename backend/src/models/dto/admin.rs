use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct AdminStatisticsDto {
    pub total_users: i64,
    pub total_courses: i64,
    pub published_courses: i64,
    pub total_scenarios: i64,
    pub active_scenarios: i64,
    pub running_labs: i64,
    pub completed_tasks: i64,
    pub submitted_flags: i64,
}

#[derive(Debug, Serialize)]
pub struct AdminDashboardDto {
    pub statistics: AdminStatisticsDto,
}

#[derive(Debug, Serialize)]
pub struct AdminUserDto {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub role: String,
    pub total_score: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminLabDto {
    pub environment_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub scenario_id: Uuid,
    pub scenario_title: String,
    pub environment_status: String,
    pub instance_id: Option<Uuid>,
    pub vm_name: Option<String>,
    pub instance_status: Option<String>,
    pub ssh_port: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize)]
pub struct AdminFlagDto {
    pub id: Uuid,
    pub scenario_id: Option<Uuid>,
    pub scenario_title: Option<String>,
    pub masked_value: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateScenarioRequest {
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub vm_template_name: String,
    pub estimated_time_minutes: i32,
    pub max_score: i32,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateScenarioRequest {
    pub title: String,
    pub description: Option<String>,
    pub difficulty: Option<String>,
    pub vm_template_name: String,
    pub estimated_time_minutes: i32,
    pub max_score: i32,
    pub is_active: bool,
}
