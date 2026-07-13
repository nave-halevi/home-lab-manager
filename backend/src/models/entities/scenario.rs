use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Scenario {
    pub id: Uuid,
    pub title: String,
    pub difficulty: Option<String>,
    pub description: Option<String>,
    pub vm_template_name: String,
    pub estimated_time_minutes: i32,
    pub max_score: i32,
    pub is_active: bool,
}
