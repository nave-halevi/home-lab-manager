use serde::{
    Deserialize,
    Serialize,
};
use uuid::Uuid;


#[derive(Debug, serde::Deserialize)]
pub struct SubmitFlagRequest {
    pub user_id: Uuid,
    pub env_id: Uuid,
    pub task_id: Uuid,
    pub flag: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateLabRequest {
    pub user_id: Uuid,
    pub scenario_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct CreateLabResponse {
    pub message: String,
    pub ssh_port: Option<u16>,
    pub env_id: Option<Uuid>,
}

#[derive(Debug, Deserialize)]
pub struct DeleteLabRequest {
    pub user_id: Uuid,
    pub env_id: Uuid,
}

#[derive(Debug, Serialize)]
pub struct DeleteLabResponse {
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct SubmitFlagResponse {
    pub message: String,
}