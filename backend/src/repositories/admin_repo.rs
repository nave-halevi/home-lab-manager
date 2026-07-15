use chrono::{DateTime, Utc};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::models::{dto::admin::AdminStatisticsDto, entities::Scenario};

#[derive(Debug, FromRow)]
pub struct AdminUserRow {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub role: String,
    pub total_score: i32,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub avatar_url: Option<String>,
}

#[derive(Debug, FromRow)]
pub struct AdminLabRow {
    pub environment_id: Uuid,
    pub user_id: Uuid,
    pub user_name: String,
    pub email: String,
    pub scenario_id: Uuid,
    pub scenario_title: String,
    pub environment_status: String,
    pub instance_id: Option<Uuid>,
    pub vm_name: Option<String>,
    pub instance_status: Option<String>,
    pub ssh_port: Option<i32>,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
}

#[derive(Debug, FromRow)]
pub struct AdminFlagRow {
    pub id: Uuid,
    pub scenario_id: Option<Uuid>,
    pub scenario_title: Option<String>,
    pub flag_value: String,
    pub points: i32,
}

async fn count(pool: &PgPool, query: &str) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(query).fetch_one(pool).await
}

pub async fn get_statistics(pool: &PgPool) -> Result<AdminStatisticsDto, sqlx::Error> {
    Ok(AdminStatisticsDto {
        total_users: count(pool, "SELECT COUNT(*) FROM users").await?,
        total_courses: count(pool, "SELECT COUNT(*) FROM courses").await?,
        published_courses: count(
            pool,
            "SELECT COUNT(*) FROM courses WHERE is_published = true",
        )
        .await?,
        total_scenarios: count(pool, "SELECT COUNT(*) FROM scenarios").await?,
        active_scenarios: count(
            pool,
            "SELECT COUNT(*) FROM scenarios WHERE is_active = true",
        )
        .await?,
        running_labs: count(
            pool,
            "SELECT COUNT(*) FROM environments WHERE status = 'Running'",
        )
        .await?,
        completed_tasks: count(
            pool,
            "SELECT COUNT(*) FROM user_task_progress WHERE status = 'COMPLETED'",
        )
        .await?,
        submitted_flags: count(pool, "SELECT COUNT(*) FROM user_flags").await?,
    })
}

pub async fn get_users(pool: &PgPool) -> Result<Vec<AdminUserRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT id, user_name, email, role, total_score,
               created_at, updated_at, avatar_url
        FROM users
        ORDER BY created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_labs(pool: &PgPool) -> Result<Vec<AdminLabRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminLabRow>(
        r#"
        SELECT e.id AS environment_id, e.user_id, u.user_name, u.email,
               e.scenario_id, s.title AS scenario_title,
               e.status AS environment_status, i.id AS instance_id,
               i.vm_name, i.status AS instance_status, i.ssh_port,
               e.created_at, e.started_at
        FROM environments e
        JOIN users u ON u.id = e.user_id
        JOIN scenarios s ON s.id = e.scenario_id
        LEFT JOIN LATERAL (
            SELECT id, vm_name, status, ssh_port
            FROM instances
            WHERE environment_id = e.id
            ORDER BY created_at DESC
            LIMIT 1
        ) i ON true
        ORDER BY e.created_at DESC
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_flags(pool: &PgPool) -> Result<Vec<AdminFlagRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminFlagRow>(
        r#"
        SELECT f.id, f.scenario_id, s.title AS scenario_title,
               f.flag_value, f.points
        FROM flags f
        LEFT JOIN scenarios s ON s.id = f.scenario_id
        ORDER BY s.title NULLS LAST, f.id
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn get_scenarios(pool: &PgPool) -> Result<Vec<Scenario>, sqlx::Error> {
    sqlx::query_as::<_, Scenario>(
        r#"
        SELECT id, title, difficulty, description, vm_template_name,
               estimated_time_minutes, max_score, is_active
        FROM scenarios
        ORDER BY title
        "#,
    )
    .fetch_all(pool)
    .await
}

pub async fn create_scenario(
    pool: &PgPool,
    title: &str,
    description: Option<&str>,
    difficulty: Option<&str>,
    vm_template_name: &str,
    estimated_time_minutes: i32,
    max_score: i32,
    is_active: bool,
) -> Result<Scenario, sqlx::Error> {
    sqlx::query_as::<_, Scenario>(
        r#"
        INSERT INTO scenarios
            (title, description, difficulty, vm_template_name,
             estimated_time_minutes, max_score, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id, title, difficulty, description, vm_template_name,
                  estimated_time_minutes, max_score, is_active
        "#,
    )
    .bind(title)
    .bind(description)
    .bind(difficulty)
    .bind(vm_template_name)
    .bind(estimated_time_minutes)
    .bind(max_score)
    .bind(is_active)
    .fetch_one(pool)
    .await
}

#[allow(clippy::too_many_arguments)]
pub async fn update_scenario(
    pool: &PgPool,
    id: Uuid,
    title: &str,
    description: Option<&str>,
    difficulty: Option<&str>,
    vm_template_name: &str,
    estimated_time_minutes: i32,
    max_score: i32,
    is_active: bool,
) -> Result<Option<Scenario>, sqlx::Error> {
    sqlx::query_as::<_, Scenario>(
        r#"
        UPDATE scenarios
        SET title = $2, description = $3, difficulty = $4,
            vm_template_name = $5, estimated_time_minutes = $6,
            max_score = $7, is_active = $8
        WHERE id = $1
        RETURNING id, title, difficulty, description, vm_template_name,
                  estimated_time_minutes, max_score, is_active
        "#,
    )
    .bind(id)
    .bind(title)
    .bind(description)
    .bind(difficulty)
    .bind(vm_template_name)
    .bind(estimated_time_minutes)
    .bind(max_score)
    .bind(is_active)
    .fetch_optional(pool)
    .await
}

pub async fn scenario_reference_count(pool: &PgPool, id: Uuid) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT
          (SELECT COUNT(*) FROM tasks WHERE scenario_id = $1) +
          (SELECT COUNT(*) FROM environments WHERE scenario_id = $1) +
          (SELECT COUNT(*) FROM flags WHERE scenario_id = $1)
        "#,
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

pub async fn delete_scenario(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM scenarios WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await?;
    Ok(result.rows_affected() == 1)
}
