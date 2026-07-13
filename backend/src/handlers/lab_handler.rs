use axum::{
    Json,
    extract::{Extension, Path, State},
    http::StatusCode,
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::{
        dto::lab::LabStatusResponseDto,
        lab_models::{
            CreateLabRequest, CreateLabResponse, DeleteLabRequest, DeleteLabResponse,
            SubmitFlagRequest, SubmitFlagResponse,
        },
        user::Claims,
    },
    services::instance_service,
};

fn user_id_from_claims(claims: &Claims) -> Result<Uuid, StatusCode> {
    Uuid::parse_str(&claims.sub).map_err(|_| StatusCode::UNAUTHORIZED)
}

pub async fn handle_create_lab(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<CreateLabRequest>,
) -> (StatusCode, Json<CreateLabResponse>) {
    let user_id = match user_id_from_claims(&claims) {
        Ok(user_id) => user_id,

        Err(status) => {
            return (
                status,
                Json(CreateLabResponse {
                    message: "Unauthorized".to_string(),
                    ssh_port: None,
                    env_id: None,
                }),
            );
        }
    };

    match instance_service::create_user_lab(&pool, user_id, payload.scenario_id).await {
        Ok(lab) => (
            StatusCode::CREATED,
            Json(CreateLabResponse {
                message: "The lab was set up and is running successfully!".to_string(),
                ssh_port: lab.ssh_port.and_then(|port| u16::try_from(port).ok()),
                env_id: Some(lab.environment_id),
            }),
        ),

        Err(error) => {
            eprintln!("[ERROR] Failed to create lab in handler: {}", error);

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
    Extension(claims): Extension<Claims>,
    Json(payload): Json<DeleteLabRequest>,
) -> (StatusCode, Json<DeleteLabResponse>) {
    let user_id = match user_id_from_claims(&claims) {
        Ok(user_id) => user_id,

        Err(status) => {
            return (
                status,
                Json(DeleteLabResponse {
                    message: "Unauthorized".to_string(),
                }),
            );
        }
    };

    match instance_service::delete_user_lab(&pool, user_id, payload.env_id).await {
        Ok(()) => (
            StatusCode::OK,
            Json(DeleteLabResponse {
                message: "The laboratory was deleted successfully.".to_string(),
            }),
        ),

        Err(error) => {
            eprintln!("[ERROR] Failed to delete lab in handler: {}", error);

            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(DeleteLabResponse {
                    message: format!("Failed to delete laboratory: {}", error),
                }),
            )
        }
    }
}

pub async fn handle_submit_flag(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<SubmitFlagRequest>,
) -> (StatusCode, Json<SubmitFlagResponse>) {
    let user_id = match user_id_from_claims(&claims) {
        Ok(user_id) => user_id,

        Err(status) => {
            return (
                status,
                Json(SubmitFlagResponse {
                    message: "Unauthorized".to_string(),
                }),
            );
        }
    };

    match instance_service::verify_and_submit_flag(
        &pool,
        user_id,
        payload.env_id,
        payload.task_id,
        &payload.flag,
    )
    .await
    {
        Ok(message) => (StatusCode::OK, Json(SubmitFlagResponse { message })),

        Err(error) => {
            eprintln!("[ERROR] Failed to submit flag in handler: {}", error);

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
    Extension(claims): Extension<Claims>,
    Path(environment_id): Path<Uuid>,
) -> (StatusCode, Json<Option<LabStatusResponseDto>>) {
    let user_id = match user_id_from_claims(&claims) {
        Ok(user_id) => user_id,

        Err(status) => {
            return (status, Json(None));
        }
    };

    match instance_service::get_lab_status(&pool, user_id, environment_id).await {
        Ok(response) => (StatusCode::OK, Json(Some(response))),

        Err(error) => {
            eprintln!("[ERROR] Failed to retrieve lab status: {}", error);

            (StatusCode::NOT_FOUND, Json(None))
        }
    }
}

pub async fn handle_get_active_lab(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(scenario_id): Path<Uuid>,
) -> (StatusCode, Json<Option<LabStatusResponseDto>>) {
    let user_id = match user_id_from_claims(&claims) {
        Ok(user_id) => user_id,

        Err(status) => {
            return (status, Json(None));
        }
    };

    match instance_service::get_active_lab(&pool, user_id, scenario_id).await {
        Ok(response) => (StatusCode::OK, Json(response)),

        Err(error) => {
            eprintln!("[ERROR] Failed to retrieve active lab: {}", error);

            (StatusCode::INTERNAL_SERVER_ERROR, Json(None))
        }
    }
}
