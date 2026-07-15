use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{entities::Environment, status::EnvironmentStatus};

pub async fn create_environment(
    pool: &PgPool,
    user_id: Uuid,
    scenario_id: Uuid,
    expires_at: DateTime<Utc>,
) -> Result<Environment, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        INSERT INTO environments (
            user_id,
            scenario_id,
            status,
            expires_at
        )
        VALUES ($1, $2, $3, $4)
        RETURNING
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity,
            expires_at
        "#,
        user_id,
        scenario_id,
        EnvironmentStatus::Building.as_str(),
        expires_at
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
            last_activity,
            expires_at
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
            last_activity,
            expires_at
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
            last_activity,
            expires_at
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
            last_activity = now(),
            expires_at = CASE
                WHEN $3::text IN ('Building', 'Running', 'Stopping')
                    THEN expires_at
                ELSE NULL
            END
        WHERE id = $1
        RETURNING
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity,
            expires_at
        "#,
        environment_id,
        status.as_str(),
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
            last_activity = now(),
            expires_at = NULL
        WHERE id = $1
        RETURNING
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity,
            expires_at
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
            last_activity,
            expires_at
        FROM environments
        WHERE id = $1
        "#,
        environment_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn find_any_active_environment(
    pool: &PgPool,
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
            last_activity,
            expires_at
        FROM environments
        WHERE user_id = $1
          AND status IN ('Building', 'Running', 'Stopping')
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn touch_environment_activity(
    pool: &PgPool,
    environment_id: Uuid,
    expires_at: DateTime<Utc>,
) -> Result<Environment, sqlx::Error> {
    sqlx::query_as!(
        Environment,
        r#"
        UPDATE environments
        SET last_activity = now(),
            expires_at = $2
        WHERE id = $1
          AND status IN ('Building', 'Running', 'Stopping')
        RETURNING
            id,
            user_id,
            scenario_id,
            status,
            created_at,
            started_at,
            stopped_at,
            last_activity,
            expires_at
        "#,
        environment_id,
        expires_at
    )
    .fetch_one(pool)
    .await
}

pub async fn find_expired_active_environments(
    pool: &PgPool,
    limit: i64,
) -> Result<Vec<Environment>, sqlx::Error> {
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
            last_activity,
            expires_at
        FROM environments
        WHERE status IN ('Building', 'Running', 'Stopping')
          AND expires_at IS NOT NULL
          AND expires_at <= now()
        ORDER BY expires_at ASC
        LIMIT $1
        "#,
        limit
    )
    .fetch_all(pool)
    .await
}
