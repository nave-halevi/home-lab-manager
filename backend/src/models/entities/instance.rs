use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Instance {
    pub id: Uuid,
    pub environment_id: Uuid,
    pub vm_name: String,
    pub is_entry_point: bool,
    pub internal_ip: Option<String>,
    pub created_at: DateTime<Utc>,
    pub ssh_port: Option<i32>,
    pub status: String,
    pub last_activity: DateTime<Utc>,
}
