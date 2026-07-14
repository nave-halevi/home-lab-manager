use axum::{
    Json,
    extract::{Extension, State},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{dto::dashboard::DashboardResponseDto, user::Claims},
    services::dashboard_service,
};

pub async fn get_dashboard_handler(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
) -> Result<Json<DashboardResponseDto>, AppError> {
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| AppError::Unauthorized)?;

    let dashboard = dashboard_service::get_dashboard(&pool, user_id).await?;

    Ok(Json(dashboard))
}
