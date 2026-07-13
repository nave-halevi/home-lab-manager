use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{entities::Environment, status::EnvironmentStatus};

pub async fn create_environment(
    pool: &PgPool,
    user_id: Uuid,
    scenario_id: Uuid,
) -> Result<Environment, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        INSERT INTO environments (
            user_id,
            scenario_id,
            status
        )
        VALUES ($1, $2, $3)
        RETURNING
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity
        "#,
        user_id,
        scenario_id,
        EnvironmentStatus::Building.as_str()
    )
    .fetch_one(pool)
    .await
}

pub async fn find_active_environment(
    pool: &PgPool,
    user_id: Uuid,
    scenario_id: Uuid,
) -> Result<Option<Environment>, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        SELECT
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity
        FROM environments
        WHERE user_id = $1
          AND scenario_id = $2
          AND status IN ('Building', 'Running', 'Stopping')
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        user_id,
        scenario_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_user_environment_by_id(
    pool: &PgPool,
    environment_id: Uuid,
    user_id: Uuid,
) -> Result<Option<Environment>, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        SELECT
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity
        FROM environments
        WHERE id = $1
          AND user_id = $2
        "#,
        environment_id,
        user_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn mark_environment_running(
    pool: &PgPool,
    environment_id: Uuid,
) -> Result<Environment, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        UPDATE environments
        SET
            status = $2,
            started_at = COALESCE(started_at, now()),
            stopped_at = NULL,
            last_activity = now()
        WHERE id = $1
        RETURNING
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity
        "#,
        environment_id,
        EnvironmentStatus::Running.as_str()
    )
    .fetch_one(pool)
    .await
}

pub async fn update_environment_status(
    pool: &PgPool,
    environment_id: Uuid,
    status: EnvironmentStatus,
) -> Result<Environment, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        UPDATE environments
        SET
            status = $2,
            last_activity = now()
        WHERE id = $1
        RETURNING
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity
        "#,
        environment_id,
        status.as_str()
    )
    .fetch_one(pool)
    .await
}

pub async fn mark_environment_destroyed(
    pool: &PgPool,
    environment_id: Uuid,
) -> Result<Environment, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        UPDATE environments
        SET
            status = $2,
            stopped_at = now(),
            last_activity = now()
        WHERE id = $1
        RETURNING
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity
        "#,
        environment_id,
        EnvironmentStatus::Destroyed.as_str()
    )
    .fetch_one(pool)
    .await
}

pub async fn find_environment_by_id(
    pool: &PgPool,
    environment_id: Uuid,
) -> Result<Option<Environment>, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        SELECT
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity
        FROM environments
        WHERE id = $1
        "#,
        environment_id
    )
    .fetch_optional(pool)
    .await
}
