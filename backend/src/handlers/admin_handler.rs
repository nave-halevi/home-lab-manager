use axum::{
    Json,
    extract::{Path, State},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        dto::admin::{
            AdminDashboardDto, AdminFlagDto, AdminLabDto, AdminUserDto, CreateScenarioRequest,
            UpdateScenarioRequest,
        },
        entities::Scenario,
    },
    services::admin_service,
};

pub async fn dashboard(State(pool): State<PgPool>) -> Result<Json<AdminDashboardDto>, AppError> {
    Ok(Json(admin_service::get_dashboard(&pool).await?))
}
pub async fn users(State(pool): State<PgPool>) -> Result<Json<Vec<AdminUserDto>>, AppError> {
    Ok(Json(admin_service::get_users(&pool).await?))
}
pub async fn labs(State(pool): State<PgPool>) -> Result<Json<Vec<AdminLabDto>>, AppError> {
    Ok(Json(admin_service::get_labs(&pool).await?))
}
pub async fn flags(State(pool): State<PgPool>) -> Result<Json<Vec<AdminFlagDto>>, AppError> {
    Ok(Json(admin_service::get_flags(&pool).await?))
}
pub async fn scenarios(State(pool): State<PgPool>) -> Result<Json<Vec<Scenario>>, AppError> {
    Ok(Json(admin_service::get_scenarios(&pool).await?))
}
pub async fn create_scenario(
    State(pool): State<PgPool>,
    Json(req): Json<CreateScenarioRequest>,
) -> Result<Json<Scenario>, AppError> {
    Ok(Json(admin_service::create_scenario(&pool, req).await?))
}
pub async fn update_scenario(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateScenarioRequest>,
) -> Result<Json<Scenario>, AppError> {
    Ok(Json(admin_service::update_scenario(&pool, id, req).await?))
}
pub async fn delete_scenario(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    admin_service::delete_scenario(&pool, id).await
}
