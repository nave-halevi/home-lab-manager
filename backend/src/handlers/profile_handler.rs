use axum::{
    Json,
    extract::{Extension, State},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        dto::profile::{
            ChangePasswordRequest, MessageResponseDto, ProfileResponseDto, UpdateAvatarRequest,
            UpdateProfileRequest,
        },
        user::Claims,
    },
    services::profile_service,
};

pub async fn get_profile_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<ProfileResponseDto>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

    let profile = profile_service::get_profile(&pool, user_id).await?;

    Ok(Json(profile))
}

pub async fn update_profile_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<ProfileResponseDto>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;
    let profile = profile_service::update_profile(&pool, user_id, payload).await?;

    Ok(Json(profile))
}

pub async fn change_password_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<Json<MessageResponseDto>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;
    let response = profile_service::change_password(&pool, user_id, payload).await?;

    Ok(Json(response))
}

pub async fn update_avatar_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(payload): Json<UpdateAvatarRequest>,
) -> Result<Json<ProfileResponseDto>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;
    let profile = profile_service::update_avatar(&pool, user_id, payload).await?;

    Ok(Json(profile))
}
