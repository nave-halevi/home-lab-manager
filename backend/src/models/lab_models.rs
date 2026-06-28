use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct CreateLabRequest {
    pub scenario_id: Uuid,
    pub user_id: Uuid,
}

#[derive(Serialize)]
pub struct CreateLabResponse {
    pub message: String,
    pub ssh_port: Option<u16>,
    pub env_id: Option<Uuid>,
}

#[derive(Deserialize)]
pub struct DeleteLabRequest {
    pub env_id: Uuid,
}

#[derive(Serialize)]
pub struct DeleteLabResponse {
    pub message: String,
}

#[derive(Deserialize)]
pub struct SubmitFlagRequest {
    pub env_id: Uuid,
    pub flag: String,
}

#[derive(Serialize)]
pub struct SubmitFlagResponse {
    pub message: String,
}