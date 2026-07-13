use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct CreateLabResponseDto {
    pub environment_id: Uuid,
    pub instance_id: Uuid,
    pub scenario_id: Uuid,
    pub vm_name: String,
    pub ssh_port: Option<i32>,
    pub status: String,
}
