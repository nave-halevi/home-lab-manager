use bcrypt::{DEFAULT_COST, hash};
use serde_json::{Value, json};
use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    errors::AppError,
    models::{
        dto::admin::{
            AdminActivityDto, AdminDashboardDto, AdminFlagDetailsDto, AdminFlagDto, AdminLabDto,
            AdminMessageDto, AdminPasswordResetRequest, AdminUserActivitySummaryDto,
            AdminUserDetailsDto, AdminUserDto, CreateFlagRequest, CreateScenarioRequest,
            PaginatedResponse, PaginationQuery, UpdateFlagRequest, UpdateScenarioRequest,
            UpdateUserRoleRequest, UpdateUserStatusRequest,
        },
        entities::Scenario,
        status::EnvironmentStatus,
    },
    repositories::{admin_repo, environment_repo},
    services::instance_service,
};

const DEFAULT_PAGE_SIZE: i64 = 20;
const MAX_PAGE_SIZE: i64 = 100;

fn normalize_pagination(query: &PaginationQuery) -> Result<(i64, i64, i64), AppError> {
    let page = query.page.unwrap_or(1);
    let page_size = query.page_size.unwrap_or(DEFAULT_PAGE_SIZE);

    if page < 1 {
        return Err(AppError::Validation("Page must be at least 1.".to_string()));
    }
    if !(1..=MAX_PAGE_SIZE).contains(&page_size) {
        return Err(AppError::Validation(format!(
            "Page size must be between 1 and {MAX_PAGE_SIZE}."
        )));
    }

    Ok((page, page_size, (page - 1) * page_size))
}

fn paginated<T>(
    items: Vec<T>,
    page: i64,
    page_size: i64,
    total_items: i64,
) -> PaginatedResponse<T> {
    let total_pages = if total_items == 0 {
        0
    } else {
        (total_items + page_size - 1) / page_size
    };

    PaginatedResponse {
        items,
        page,
        page_size,
        total_items,
        total_pages,
    }
}

fn normalized_search(value: Option<&String>) -> Option<&str> {
    value
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
}

fn normalized_status(value: Option<&String>, allowed: &[&str]) -> Result<Option<String>, AppError> {
    let Some(value) = value
        .map(|value| value.trim())
        .filter(|value| !value.is_empty())
    else {
        return Ok(None);
    };

    let normalized = value.to_lowercase();
    if allowed.contains(&normalized.as_str()) {
        Ok(Some(normalized))
    } else {
        Err(AppError::Validation("Invalid status filter.".to_string()))
    }
}

fn display_role(role: &str) -> String {
    if role.eq_ignore_ascii_case("admin") {
        "Admin".to_string()
    } else {
        "User".to_string()
    }
}

fn user_dto(row: admin_repo::AdminUserRow) -> AdminUserDto {
    AdminUserDto {
        id: row.id,
        user_name: row.user_name,
        email: row.email,
        role: display_role(&row.role),
        total_score: row.total_score,
        created_at: row.created_at,
        updated_at: row.updated_at,
        avatar_url: row.avatar_url,
        is_active: row.is_active,
    }
}

fn lab_dto(row: admin_repo::AdminLabRow) -> AdminLabDto {
    AdminLabDto {
        environment_id: row.environment_id,
        user_id: row.user_id,
        user_name: row.user_name,
        email: row.email,
        scenario_id: row.scenario_id,
        scenario_title: row.scenario_title,
        environment_status: row.environment_status,
        instance_id: row.instance_id,
        vm_name: row.vm_name,
        instance_status: row.instance_status,
        ssh_port: row.ssh_port,
        created_at: row.created_at,
        started_at: row.started_at,
    }
}

fn mask_flag_value(flag_value: &str) -> String {
    if flag_value.starts_with("CTF{") {
        "CTF{********}".to_string()
    } else {
        "********".to_string()
    }
}

fn flag_dto(row: admin_repo::AdminFlagRow) -> AdminFlagDto {
    AdminFlagDto {
        id: row.id,
        scenario_id: row.scenario_id,
        scenario_title: row.scenario_title,
        masked_value: mask_flag_value(&row.flag_value),
        points: row.points,
    }
}

fn flag_details_dto(row: admin_repo::AdminFlagRow) -> AdminFlagDetailsDto {
    AdminFlagDetailsDto {
        id: row.id,
        scenario_id: row.scenario_id,
        scenario_title: row.scenario_title,
        flag_value: row.flag_value,
        points: row.points,
    }
}

fn activity_dto(row: admin_repo::AdminActivityRow) -> AdminActivityDto {
    AdminActivityDto {
        id: row.id,
        admin_user_id: row.admin_user_id,
        admin_user_name: row.admin_user_name,
        admin_email: row.admin_email,
        action: row.action,
        entity_type: row.entity_type,
        entity_id: row.entity_id,
        details: row.details,
        created_at: row.created_at,
    }
}

pub async fn record_activity(
    pool: &PgPool,
    admin_user_id: Uuid,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    details: Option<Value>,
) {
    if let Err(error) =
        admin_repo::insert_activity(pool, admin_user_id, action, entity_type, entity_id, details)
            .await
    {
        tracing::error!("Failed to record Admin activity: {:?}", error);
    }
}

fn safe_flag_details(row: &admin_repo::AdminFlagRow) -> Value {
    json!({
        "scenario_id": row.scenario_id,
        "points": row.points,
        "masked_value": mask_flag_value(&row.flag_value),
    })
}

pub async fn get_dashboard(pool: &PgPool) -> Result<AdminDashboardDto, AppError> {
    let statistics = admin_repo::get_statistics(pool).await?;
    let recent_activity = admin_repo::get_activity(pool, None, None, None, "desc", 8, 0)
        .await?
        .into_iter()
        .map(activity_dto)
        .collect();

    Ok(AdminDashboardDto {
        statistics,
        recent_activity,
    })
}

pub async fn get_users(
    pool: &PgPool,
    query: PaginationQuery,
) -> Result<PaginatedResponse<AdminUserDto>, AppError> {
    let (page, page_size, offset) = normalize_pagination(&query)?;
    let status = normalized_status(
        query.status.as_ref(),
        &["active", "disabled", "admin", "user"],
    )?;
    let search = normalized_search(query.search.as_ref());

    let total_items = admin_repo::count_users(pool, search, status.as_deref()).await?;
    let items = admin_repo::get_users(pool, search, status.as_deref(), page_size, offset)
        .await?
        .into_iter()
        .map(user_dto)
        .collect();

    Ok(paginated(items, page, page_size, total_items))
}

pub async fn get_user_details(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<AdminUserDetailsDto, AppError> {
    let user = admin_repo::get_user_by_id(pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;
    let activity = admin_repo::get_user_activity_summary(pool, user_id).await?;
    let recent_labs = admin_repo::get_user_recent_labs(pool, user_id, 10)
        .await?
        .into_iter()
        .map(lab_dto)
        .collect();

    Ok(AdminUserDetailsDto {
        user: user_dto(user),
        activity: AdminUserActivitySummaryDto {
            courses_with_progress: activity.courses_with_progress,
            started_tasks: activity.started_tasks,
            completed_tasks: activity.completed_tasks,
            solved_flags: activity.solved_flags,
            active_labs: activity.active_labs,
        },
        recent_labs,
    })
}

pub async fn update_user_status(
    pool: &PgPool,
    admin_user_id: Uuid,
    user_id: Uuid,
    request: UpdateUserStatusRequest,
) -> Result<AdminUserDto, AppError> {
    if admin_user_id == user_id {
        return Err(AppError::Validation(
            "Admins cannot disable or enable themselves here.".to_string(),
        ));
    }

    let target = admin_repo::get_user_by_id(pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if !request.is_active
        && target.role == "admin"
        && admin_repo::active_admin_count(pool).await? <= 1
    {
        return Err(AppError::Validation(
            "Cannot disable the last active Admin account.".to_string(),
        ));
    }

    let updated = admin_repo::update_user_status(pool, user_id, request.is_active)
        .await?
        .ok_or(AppError::NotFound)?;

    record_activity(
        pool,
        admin_user_id,
        if request.is_active {
            "User enabled"
        } else {
            "User disabled"
        },
        "User",
        Some(user_id),
        Some(json!({ "email": updated.email, "is_active": updated.is_active })),
    )
    .await;

    Ok(user_dto(updated))
}

pub async fn update_user_role(
    pool: &PgPool,
    admin_user_id: Uuid,
    user_id: Uuid,
    request: UpdateUserRoleRequest,
) -> Result<AdminUserDto, AppError> {
    if admin_user_id == user_id {
        return Err(AppError::Validation(
            "Admins cannot change their own Admin role here.".to_string(),
        ));
    }

    let role = match request.role.trim().to_lowercase().as_str() {
        "admin" => "admin",
        "user" => "user",
        _ => {
            return Err(AppError::Validation(
                "Role must be User or Admin.".to_string(),
            ));
        }
    };

    let target = admin_repo::get_user_by_id(pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if target.role == "admin"
        && role == "user"
        && target.is_active
        && admin_repo::active_admin_count(pool).await? <= 1
    {
        return Err(AppError::Validation(
            "Cannot demote the last active Admin account.".to_string(),
        ));
    }

    let updated = admin_repo::update_user_role(pool, user_id, role)
        .await?
        .ok_or(AppError::NotFound)?;

    record_activity(
        pool,
        admin_user_id,
        if role == "admin" {
            "User promoted"
        } else {
            "User demoted"
        },
        "User",
        Some(user_id),
        Some(json!({ "email": updated.email, "role": display_role(&updated.role) })),
    )
    .await;

    Ok(user_dto(updated))
}

pub async fn reset_user_password(
    pool: &PgPool,
    admin_user_id: Uuid,
    user_id: Uuid,
    request: AdminPasswordResetRequest,
) -> Result<AdminMessageDto, AppError> {
    if admin_user_id == user_id {
        return Err(AppError::Validation(
            "Admins should change their own password from Profile.".to_string(),
        ));
    }

    let target = admin_repo::get_user_by_id(pool, user_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if request.new_password != request.confirm_password {
        return Err(AppError::Validation(
            "Password confirmation does not match.".to_string(),
        ));
    }
    if request.new_password.len() < 8 {
        return Err(AppError::Validation(
            "Password must be at least 8 characters.".to_string(),
        ));
    }

    let hashed = hash(&request.new_password, DEFAULT_COST).map_err(|_| AppError::Internal)?;
    if !admin_repo::update_user_password(pool, user_id, &hashed).await? {
        return Err(AppError::NotFound);
    }

    record_activity(
        pool,
        admin_user_id,
        "User password reset",
        "User",
        Some(user_id),
        Some(json!({ "email": target.email })),
    )
    .await;

    Ok(AdminMessageDto {
        message: "Password reset successfully.".to_string(),
    })
}

pub async fn get_labs(
    pool: &PgPool,
    query: PaginationQuery,
) -> Result<PaginatedResponse<AdminLabDto>, AppError> {
    let (page, page_size, offset) = normalize_pagination(&query)?;
    let status = normalized_status(
        query.status.as_ref(),
        &["building", "running", "stopping", "failed", "destroyed"],
    )?
    .map(|value| {
        let mut chars = value.chars();
        match chars.next() {
            Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
            None => value,
        }
    });
    let search = normalized_search(query.search.as_ref());

    let total_items = admin_repo::count_labs(pool, search, status.as_deref()).await?;
    let items = admin_repo::get_labs(pool, search, status.as_deref(), page_size, offset)
        .await?
        .into_iter()
        .map(lab_dto)
        .collect();

    Ok(paginated(items, page, page_size, total_items))
}

pub async fn terminate_lab(
    pool: &PgPool,
    admin_user_id: Uuid,
    environment_id: Uuid,
) -> Result<AdminMessageDto, AppError> {
    let environment = environment_repo::find_environment_by_id(pool, environment_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if environment.status == EnvironmentStatus::Destroyed.as_str() {
        return Ok(AdminMessageDto {
            message: "Lab is already destroyed.".to_string(),
        });
    }

    if !matches!(
        environment.status.as_str(),
        "Building" | "Running" | "Failed"
    ) {
        return Err(AppError::Validation(
            "Only Building, Running, or Failed labs can be terminated.".to_string(),
        ));
    }

    instance_service::delete_user_lab(pool, environment.user_id, environment_id)
        .await
        .map_err(|error| AppError::Validation(format!("Lab termination failed: {error}")))?;

    record_activity(
        pool,
        admin_user_id,
        "Lab terminated",
        "Environment",
        Some(environment_id),
        Some(json!({
            "user_id": environment.user_id,
            "scenario_id": environment.scenario_id,
            "previous_status": environment.status,
        })),
    )
    .await;

    Ok(AdminMessageDto {
        message: "Lab terminated successfully.".to_string(),
    })
}

pub async fn get_flags(
    pool: &PgPool,
    query: PaginationQuery,
) -> Result<PaginatedResponse<AdminFlagDto>, AppError> {
    let (page, page_size, offset) = normalize_pagination(&query)?;
    let search = normalized_search(query.search.as_ref());

    let total_items = admin_repo::count_flags(pool, search, query.scenario_id).await?;
    let items = admin_repo::get_flags(pool, search, query.scenario_id, page_size, offset)
        .await?
        .into_iter()
        .map(flag_dto)
        .collect();

    Ok(paginated(items, page, page_size, total_items))
}

pub async fn get_flag(pool: &PgPool, flag_id: Uuid) -> Result<AdminFlagDetailsDto, AppError> {
    admin_repo::get_flag_by_id(pool, flag_id)
        .await?
        .map(flag_details_dto)
        .ok_or(AppError::NotFound)
}

async fn validate_flag_request(
    pool: &PgPool,
    scenario_id: Uuid,
    flag_value: &str,
    points: i32,
    except_flag_id: Option<Uuid>,
    require_active_scenario: bool,
) -> Result<(), AppError> {
    if flag_value.trim().is_empty() {
        return Err(AppError::Validation("Flag value is required.".to_string()));
    }
    if points < 0 {
        return Err(AppError::Validation(
            "Flag points cannot be negative.".to_string(),
        ));
    }

    let scenario = admin_repo::get_scenario_by_id(pool, scenario_id)
        .await?
        .ok_or_else(|| AppError::Validation("Selected scenario does not exist.".to_string()))?;

    if require_active_scenario && !scenario.is_active {
        return Err(AppError::Validation(
            "Flags cannot be created for inactive scenarios.".to_string(),
        ));
    }

    if admin_repo::flag_exists_for_scenario(pool, scenario_id, flag_value.trim(), except_flag_id)
        .await?
    {
        return Err(AppError::Validation(
            "A flag with this value already exists for the selected scenario.".to_string(),
        ));
    }

    Ok(())
}

pub async fn create_flag(
    pool: &PgPool,
    admin_user_id: Uuid,
    request: CreateFlagRequest,
) -> Result<AdminFlagDetailsDto, AppError> {
    validate_flag_request(
        pool,
        request.scenario_id,
        &request.flag_value,
        request.points,
        None,
        true,
    )
    .await?;

    let flag = admin_repo::create_flag(
        pool,
        request.scenario_id,
        request.flag_value.trim(),
        request.points,
    )
    .await?;

    record_activity(
        pool,
        admin_user_id,
        "Flag created",
        "Flag",
        Some(flag.id),
        Some(safe_flag_details(&flag)),
    )
    .await;

    Ok(flag_details_dto(flag))
}

pub async fn update_flag(
    pool: &PgPool,
    admin_user_id: Uuid,
    flag_id: Uuid,
    request: UpdateFlagRequest,
) -> Result<AdminFlagDetailsDto, AppError> {
    let existing = admin_repo::get_flag_by_id(pool, flag_id)
        .await?
        .ok_or(AppError::NotFound)?;

    validate_flag_request(
        pool,
        request.scenario_id,
        &request.flag_value,
        request.points,
        Some(flag_id),
        false,
    )
    .await?;

    let flag = admin_repo::update_flag(
        pool,
        flag_id,
        request.scenario_id,
        request.flag_value.trim(),
        request.points,
    )
    .await?
    .ok_or(AppError::NotFound)?;

    record_activity(
        pool,
        admin_user_id,
        "Flag updated",
        "Flag",
        Some(flag_id),
        Some(json!({
            "previous": safe_flag_details(&existing),
            "updated": safe_flag_details(&flag),
        })),
    )
    .await;

    Ok(flag_details_dto(flag))
}

pub async fn delete_flag(
    pool: &PgPool,
    admin_user_id: Uuid,
    flag_id: Uuid,
) -> Result<(), AppError> {
    let flag = admin_repo::get_flag_by_id(pool, flag_id)
        .await?
        .ok_or(AppError::NotFound)?;

    if admin_repo::solved_flag_count(pool, flag_id).await? > 0 {
        return Err(AppError::Validation(
            "Cannot delete this flag because users have already submitted it.".to_string(),
        ));
    }

    match admin_repo::delete_flag(pool, flag_id).await {
        Ok(true) => {
            record_activity(
                pool,
                admin_user_id,
                "Flag deleted",
                "Flag",
                Some(flag_id),
                Some(safe_flag_details(&flag)),
            )
            .await;
            Ok(())
        }
        Ok(false) => Err(AppError::NotFound),
        Err(sqlx::Error::Database(database_error)) if database_error.is_foreign_key_violation() => {
            Err(AppError::Validation(
                "Cannot delete this flag because users have already submitted it.".to_string(),
            ))
        }
        Err(error) => Err(AppError::Database(error)),
    }
}

pub async fn get_scenarios(pool: &PgPool) -> Result<Vec<Scenario>, AppError> {
    Ok(admin_repo::get_scenarios(pool).await?)
}

fn validate_scenario(
    title: &str,
    vm_template_name: &str,
    estimated_time_minutes: i32,
    max_score: i32,
) -> Result<(), AppError> {
    if title.trim().len() < 3 || title.trim().len() > 255 {
        return Err(AppError::Validation(
            "Scenario title must be between 3 and 255 characters.".to_string(),
        ));
    }
    if vm_template_name.trim().is_empty() {
        return Err(AppError::Validation(
            "VM template name is required.".to_string(),
        ));
    }
    if estimated_time_minutes < 1 || max_score < 0 {
        return Err(AppError::Validation(
            "Estimated time must be positive and max score cannot be negative.".to_string(),
        ));
    }
    Ok(())
}

pub async fn create_scenario(
    pool: &PgPool,
    admin_user_id: Uuid,
    request: CreateScenarioRequest,
) -> Result<Scenario, AppError> {
    validate_scenario(
        &request.title,
        &request.vm_template_name,
        request.estimated_time_minutes,
        request.max_score,
    )?;
    let scenario = admin_repo::create_scenario(
        pool,
        request.title.trim(),
        request.description.as_deref(),
        request.difficulty.as_deref(),
        request.vm_template_name.trim(),
        request.estimated_time_minutes,
        request.max_score,
        request.is_active,
    )
    .await?;

    record_activity(
        pool,
        admin_user_id,
        "Scenario created",
        "Scenario",
        Some(scenario.id),
        Some(json!({ "title": scenario.title, "is_active": scenario.is_active })),
    )
    .await;

    Ok(scenario)
}

pub async fn update_scenario(
    pool: &PgPool,
    admin_user_id: Uuid,
    id: Uuid,
    request: UpdateScenarioRequest,
) -> Result<Scenario, AppError> {
    validate_scenario(
        &request.title,
        &request.vm_template_name,
        request.estimated_time_minutes,
        request.max_score,
    )?;
    let scenario = admin_repo::update_scenario(
        pool,
        id,
        request.title.trim(),
        request.description.as_deref(),
        request.difficulty.as_deref(),
        request.vm_template_name.trim(),
        request.estimated_time_minutes,
        request.max_score,
        request.is_active,
    )
    .await?
    .ok_or(AppError::NotFound)?;

    record_activity(
        pool,
        admin_user_id,
        if scenario.is_active {
            "Scenario updated"
        } else {
            "Scenario deactivated"
        },
        "Scenario",
        Some(scenario.id),
        Some(json!({ "title": scenario.title, "is_active": scenario.is_active })),
    )
    .await;

    Ok(scenario)
}

pub async fn delete_scenario(pool: &PgPool, admin_user_id: Uuid, id: Uuid) -> Result<(), AppError> {
    if admin_repo::scenario_reference_count(pool, id).await? > 0 {
        return Err(AppError::Validation(
            "Scenario cannot be deleted while tasks, labs, or flags reference it. Deactivate it instead."
                .to_string(),
        ));
    }
    match admin_repo::delete_scenario(pool, id).await {
        Ok(true) => {
            record_activity(
                pool,
                admin_user_id,
                "Scenario deleted",
                "Scenario",
                Some(id),
                None,
            )
            .await;
            Ok(())
        }
        Ok(false) => Err(AppError::NotFound),
        Err(sqlx::Error::Database(database_error)) if database_error.is_foreign_key_violation() => {
            Err(AppError::Validation(
                "Scenario is now referenced and cannot be deleted. Deactivate it instead."
                    .to_string(),
            ))
        }
        Err(error) => Err(AppError::Database(error)),
    }
}

pub async fn get_activity(
    pool: &PgPool,
    query: PaginationQuery,
) -> Result<PaginatedResponse<AdminActivityDto>, AppError> {
    let (page, page_size, offset) = normalize_pagination(&query)?;
    let order = match query
        .order
        .as_deref()
        .unwrap_or("desc")
        .to_lowercase()
        .as_str()
    {
        "asc" => "asc",
        "desc" => "desc",
        _ => return Err(AppError::Validation("Invalid activity order.".to_string())),
    };
    let action = normalized_search(query.action.as_ref());
    let entity_type = normalized_search(query.entity_type.as_ref());

    let total_items =
        admin_repo::count_activity(pool, query.admin_user_id, action, entity_type).await?;
    let items = admin_repo::get_activity(
        pool,
        query.admin_user_id,
        action,
        entity_type,
        order,
        page_size,
        offset,
    )
    .await?
    .into_iter()
    .map(activity_dto)
    .collect();

    Ok(paginated(items, page, page_size, total_items))
}
