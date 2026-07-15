use axum::{
    Extension, Json,
    extract::{Path, Query, State},
};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        dto::admin::{
            AdminActivityDto, AdminDashboardDto, AdminFlagDetailsDto, AdminFlagDto, AdminLabDto,
            AdminMessageDto, AdminPasswordResetRequest, AdminUserDetailsDto, AdminUserDto,
            CreateFlagRequest, CreateScenarioRequest, PaginatedResponse, PaginationQuery,
            UpdateFlagRequest, UpdateScenarioRequest, UpdateUserRoleRequest,
            UpdateUserStatusRequest,
        },
        entities::Scenario,
        user::Claims,
    },
    services::admin_service,
};

fn admin_id(claims: &Claims) -> Result<Uuid, AppError> {
    claims.sub.parse().map_err(|_| AppError::Unauthorized)
}

pub async fn dashboard(State(pool): State<PgPool>) -> Result<Json<AdminDashboardDto>, AppError> {
    Ok(Json(admin_service::get_dashboard(&pool).await?))
}

pub async fn users(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<AdminUserDto>>, AppError> {
    Ok(Json(admin_service::get_users(&pool, query).await?))
}

pub async fn user_details(
    State(pool): State<PgPool>,
    Path(user_id): Path<Uuid>,
) -> Result<Json<AdminUserDetailsDto>, AppError> {
    Ok(Json(admin_service::get_user_details(&pool, user_id).await?))
}

pub async fn update_user_status(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateUserStatusRequest>,
) -> Result<Json<AdminUserDto>, AppError> {
    Ok(Json(
        admin_service::update_user_status(&pool, admin_id(&claims)?, user_id, req).await?,
    ))
}

pub async fn update_user_role(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<UpdateUserRoleRequest>,
) -> Result<Json<AdminUserDto>, AppError> {
    Ok(Json(
        admin_service::update_user_role(&pool, admin_id(&claims)?, user_id, req).await?,
    ))
}

pub async fn reset_user_password(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(user_id): Path<Uuid>,
    Json(req): Json<AdminPasswordResetRequest>,
) -> Result<Json<AdminMessageDto>, AppError> {
    Ok(Json(
        admin_service::reset_user_password(&pool, admin_id(&claims)?, user_id, req).await?,
    ))
}

pub async fn labs(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<AdminLabDto>>, AppError> {
    Ok(Json(admin_service::get_labs(&pool, query).await?))
}

pub async fn terminate_lab(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(environment_id): Path<Uuid>,
) -> Result<Json<AdminMessageDto>, AppError> {
    Ok(Json(
        admin_service::terminate_lab(&pool, admin_id(&claims)?, environment_id).await?,
    ))
}

pub async fn flags(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<AdminFlagDto>>, AppError> {
    Ok(Json(admin_service::get_flags(&pool, query).await?))
}

pub async fn flag(
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<Json<AdminFlagDetailsDto>, AppError> {
    Ok(Json(admin_service::get_flag(&pool, id).await?))
}

pub async fn create_flag(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateFlagRequest>,
) -> Result<Json<AdminFlagDetailsDto>, AppError> {
    Ok(Json(
        admin_service::create_flag(&pool, admin_id(&claims)?, req).await?,
    ))
}

pub async fn update_flag(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateFlagRequest>,
) -> Result<Json<AdminFlagDetailsDto>, AppError> {
    Ok(Json(
        admin_service::update_flag(&pool, admin_id(&claims)?, id, req).await?,
    ))
}

pub async fn delete_flag(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    admin_service::delete_flag(&pool, admin_id(&claims)?, id).await
}

pub async fn scenarios(State(pool): State<PgPool>) -> Result<Json<Vec<Scenario>>, AppError> {
    Ok(Json(admin_service::get_scenarios(&pool).await?))
}

pub async fn create_scenario(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Json(req): Json<CreateScenarioRequest>,
) -> Result<Json<Scenario>, AppError> {
    Ok(Json(
        admin_service::create_scenario(&pool, admin_id(&claims)?, req).await?,
    ))
}

pub async fn update_scenario(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateScenarioRequest>,
) -> Result<Json<Scenario>, AppError> {
    Ok(Json(
        admin_service::update_scenario(&pool, admin_id(&claims)?, id, req).await?,
    ))
}

pub async fn delete_scenario(
    State(pool): State<PgPool>,
    Extension(claims): Extension<Claims>,
    Path(id): Path<Uuid>,
) -> Result<(), AppError> {
    admin_service::delete_scenario(&pool, admin_id(&claims)?, id).await
}

pub async fn activity(
    State(pool): State<PgPool>,
    Query(query): Query<PaginationQuery>,
) -> Result<Json<PaginatedResponse<AdminActivityDto>>, AppError> {
    Ok(Json(admin_service::get_activity(&pool, query).await?))
}
