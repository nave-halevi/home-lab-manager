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
    repositories::admin_repo,
};

pub async fn get_dashboard(pool: &PgPool) -> Result<AdminDashboardDto, AppError> {
    Ok(AdminDashboardDto {
        statistics: admin_repo::get_statistics(pool).await?,
    })
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<AdminUserDto>, AppError> {
    Ok(admin_repo::get_users(pool)
        .await?
        .into_iter()
        .map(|row| AdminUserDto {
            id: row.id,
            user_name: row.user_name,
            email: row.email,
            role: row.role,
            total_score: row.total_score,
            created_at: row.created_at,
            updated_at: row.updated_at,
            avatar_url: row.avatar_url,
        })
        .collect())
}

pub async fn get_labs(pool: &PgPool) -> Result<Vec<AdminLabDto>, AppError> {
    Ok(admin_repo::get_labs(pool)
        .await?
        .into_iter()
        .map(|row| AdminLabDto {
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
        })
        .collect())
}

pub async fn get_flags(pool: &PgPool) -> Result<Vec<AdminFlagDto>, AppError> {
    Ok(admin_repo::get_flags(pool)
        .await?
        .into_iter()
        .map(|row| AdminFlagDto {
            id: row.id,
            scenario_id: row.scenario_id,
            scenario_title: row.scenario_title,
            masked_value: if row.flag_value.starts_with("CTF{") {
                "CTF{********}".to_string()
            } else {
                "********".to_string()
            },
            points: row.points,
        })
        .collect())
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
    request: CreateScenarioRequest,
) -> Result<Scenario, AppError> {
    validate_scenario(
        &request.title,
        &request.vm_template_name,
        request.estimated_time_minutes,
        request.max_score,
    )?;
    Ok(admin_repo::create_scenario(
        pool,
        request.title.trim(),
        request.description.as_deref(),
        request.difficulty.as_deref(),
        request.vm_template_name.trim(),
        request.estimated_time_minutes,
        request.max_score,
        request.is_active,
    )
    .await?)
}

pub async fn update_scenario(
    pool: &PgPool,
    id: Uuid,
    request: UpdateScenarioRequest,
) -> Result<Scenario, AppError> {
    validate_scenario(
        &request.title,
        &request.vm_template_name,
        request.estimated_time_minutes,
        request.max_score,
    )?;
    admin_repo::update_scenario(
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
    .ok_or(AppError::NotFound)
}

pub async fn delete_scenario(pool: &PgPool, id: Uuid) -> Result<(), AppError> {
    if admin_repo::scenario_reference_count(pool, id).await? > 0 {
        return Err(AppError::Validation(
            "Scenario cannot be deleted while tasks, labs, or flags reference it. Deactivate it instead."
                .to_string(),
        ));
    }
    match admin_repo::delete_scenario(pool, id).await {
        Ok(true) => Ok(()),
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
