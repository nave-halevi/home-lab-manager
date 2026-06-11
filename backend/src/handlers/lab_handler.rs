use axum::{extract::State, Json, http::StatusCode};
use sqlx::PgPool;

use crate::models::lab_models::{CreateLabRequest, CreateLabResponse, DeleteLabRequest, DeleteLabResponse};
use crate::services::lab_service;


pub async fn handle_create_lab(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLabRequest>,
) -> (StatusCode, Json<CreateLabResponse>) {


    match lab_service::create_user_lab(&pool,payload.user_id, payload.scenario_id).await {
        Ok((env_id, port)) =>{
            (StatusCode::CREATED, Json(CreateLabResponse{
                message: "המעבדה הוקמה ורצה בהצלחה!".to_string(),
                ssh_port: Some(port),
                env_id: Some(env_id),
            }))
        }
        Err(err) => {
            eprintln!("[ERROR] Failed to create lab in handler: {}", err);
            (StatusCode::INTERNAL_SERVER_ERROR, Json(CreateLabResponse{
                message: err,
                ssh_port: None,
                env_id: None,
            }))
        }
    }
}

pub async fn handle_delete_lab(
    State(pool): State<PgPool>,
    Json(payload): Json<DeleteLabRequest>,
) -> (StatusCode, Json<DeleteLabResponse>) {

    match lab_service::delete_user_lab(&pool, payload.env_id).await {
        Ok(_) => {
            (StatusCode::OK, Json(DeleteLabResponse {
                message: "המעבדה נמחקה מהשרת בהצלחה!".to_string(),
            }))
        }
        Err(err) => {
            (StatusCode::INTERNAL_SERVER_ERROR, Json(DeleteLabResponse {
                message: format!("שגיאה: {}", err),
            }))
        }
    }
}