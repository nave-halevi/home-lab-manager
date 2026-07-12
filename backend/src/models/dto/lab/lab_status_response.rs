use chrono::{
    DateTime,
    Utc,
};
use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct LabStatusResponseDto {
    pub environment_id: Uuid,
    pub scenario_id: Uuid,
    pub environment_status: String,

    pub instance_id: Option<Uuid>,
    pub vm_name: Option<String>,
    pub ssh_port: Option<i32>,
    pub instance_status: Option<String>,
    pub is_entry_point: Option<bool>,

    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub stopped_at: Option<DateTime<Utc>>,
}