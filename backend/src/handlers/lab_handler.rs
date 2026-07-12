use axum::{
    extract::{State, Path},
    http::StatusCode,
    Json,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::dto::lab::LabStatusResponseDto;

use crate::{
    models::lab_models::{
        CreateLabRequest,
        CreateLabResponse,
        DeleteLabRequest,
        DeleteLabResponse,
        SubmitFlagRequest,
        SubmitFlagResponse,
    },
    services::instance_service,
};

pub async fn handle_create_lab(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateLabRequest>,
) -> (StatusCode, Json<CreateLabResponse>) {
    match instance_service::create_user_lab(
        &pool,
        payload.user_id,
        payload.scenario_id,
    )
    .await
    {
        Ok(lab) => (
            StatusCode::CREATED,
            Json(CreateLabResponse {
                message: "The lab was set up and is running successfully!"
                    .to_string(),
                ssh_port: lab
                    .ssh_port
                    .and_then(|port| u16::try_from(port).ok()),
                env_id: Some(lab.environment_id),
            }),
        ),

        Err(error) => {
            eprintln!(
                "[ERROR] Failed to create lab in handler: {}",
                error
            );

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CreateLabResponse {
                    message: error,
                    ssh_port: None,
                    env_id: None,
                }),
            )
        }
    }
}

pub async fn handle_delete_lab(
    State(pool): State<PgPool>,
    Json(payload): Json<DeleteLabRequest>,
) -> (StatusCode, Json<DeleteLabResponse>) {
    match instance_service::delete_user_lab(
        &pool,
        payload.user_id,
        payload.env_id,
    )
    .await
    {
        Ok(()) => (
            StatusCode::OK,
            Json(DeleteLabResponse {
                message: "The laboratory was deleted successfully."
                    .to_string(),
            }),
        ),

        Err(error) => {
            eprintln!(
                "[ERROR] Failed to delete lab in handler: {}",
                error
            );

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteLabResponse {
                    message: format!(
                        "Failed to delete laboratory: {}",
                        error
                    ),
                }),
            )
        }
    }
}

pub async fn handle_submit_flag(
    State(pool): State<PgPool>,
    Json(payload): Json<SubmitFlagRequest>,
) -> (StatusCode, Json<SubmitFlagResponse>) {
    match instance_service::verify_and_submit_flag(
        &pool,
        payload.user_id,
        payload.env_id,
        payload.task_id,
        &payload.flag,
    )
    .await
    {
        Ok(message) => (
            StatusCode::OK,
            Json(SubmitFlagResponse { message }),
        ),

        Err(error) => {
            eprintln!(
                "[ERROR] Failed to submit flag in handler: {}",
                error
            );

            (
                StatusCode::BAD_REQUEST,
                Json(SubmitFlagResponse {
                    message: format!("Error: {}", error),
                }),
            )
        }
    }
}

pub async fn handle_get_lab_status(
    State(pool): State<PgPool>,
    Path((user_id, environment_id)): Path<(Uuid, Uuid)>,
) -> (
    StatusCode,
    Json<Option<LabStatusResponseDto>>,
) {
    match instance_service::get_lab_status(
        &pool,
        user_id,
        environment_id,
    )
    .await
    {
        Ok(response) => (
            StatusCode::OK,
            Json(Some(response)),
        ),

        Err(error) => {
            eprintln!(
                "[ERROR] Failed to retrieve lab status: {}",
                error
            );

            (
                StatusCode::NOT_FOUND,
                Json(None),
            )
        }
    }
}

pub async fn handle_get_active_lab(
    State(pool): State<PgPool>,
    Path((user_id, scenario_id)): Path<(Uuid, Uuid)>,
) -> (
    StatusCode,
    Json<Option<LabStatusResponseDto>>,
) {
    match instance_service::get_active_lab(
        &pool,
        user_id,
        scenario_id,
    )
    .await
    {
        Ok(response) => (
            StatusCode::OK,
            Json(response),
        ),

        Err(error) => {
            eprintln!(
                "[ERROR] Failed to retrieve active lab: {}",
                error
            );

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(None),
            )
        }
    }
}