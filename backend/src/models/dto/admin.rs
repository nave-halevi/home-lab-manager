use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct PaginatedResponse<T> {
    pub items: Vec<T>,
    pub page: i64,
    pub page_size: i64,
    pub total_items: i64,
    pub total_pages: i64,
}

#[derive(Debug, Deserialize)]
pub struct PaginationQuery {
    pub page: Option<i64>,
    pub page_size: Option<i64>,
    pub search: Option<String>,
    pub status: Option<String>,
    pub scenario_id: Option<Uuid>,
    pub admin_user_id: Option<Uuid>,
    pub action: Option<String>,
    pub entity_type: Option<String>,
    pub order: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct AdminStatisticsDto {
    pub total_users: i64,
    pub active_users: i64,
    pub disabled_users: i64,
    pub admin_users: i64,
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
    pub recent_activity: Vec<AdminActivityDto>,
}

#[derive(Debug, Serialize, Clone)]
pub struct AdminUserDto {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub role: String,
    pub total_score: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub avatar_url: Option<String>,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct AdminUserActivitySummaryDto {
    pub courses_with_progress: i64,
    pub started_tasks: i64,
    pub completed_tasks: i64,
    pub solved_flags: i64,
    pub active_labs: i64,
}

#[derive(Debug, Serialize)]
pub struct AdminUserDetailsDto {
    pub user: AdminUserDto,
    pub activity: AdminUserActivitySummaryDto,
    pub recent_labs: Vec<AdminLabDto>,
}

#[derive(Debug, Serialize, Clone)]
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

#[derive(Debug, Serialize)]
pub struct AdminFlagDetailsDto {
    pub id: Uuid,
    pub scenario_id: Option<Uuid>,
    pub scenario_title: Option<String>,
    pub flag_value: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct CreateFlagRequest {
    pub scenario_id: Uuid,
    pub flag_value: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateFlagRequest {
    pub scenario_id: Uuid,
    pub flag_value: String,
    pub points: i32,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserStatusRequest {
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateUserRoleRequest {
    pub role: String,
}

#[derive(Debug, Deserialize)]
pub struct AdminPasswordResetRequest {
    pub new_password: String,
    pub confirm_password: String,
}

#[derive(Debug, Serialize)]
pub struct AdminMessageDto {
    pub message: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct AdminActivityDto {
    pub id: Uuid,
    pub admin_user_id: Uuid,
    pub admin_user_name: Option<String>,
    pub admin_email: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub details: Option<Value>,
    pub created_at: DateTime<Utc>,
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
