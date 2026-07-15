use chrono::{DateTime, Utc};
use serde_json::Value;
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
    pub is_active: bool,
}

#[derive(Debug, FromRow)]
pub struct AdminUserActivitySummaryRow {
    pub courses_with_progress: i64,
    pub started_tasks: i64,
    pub completed_tasks: i64,
    pub solved_flags: i64,
    pub active_labs: i64,
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

#[derive(Debug, FromRow)]
pub struct AdminActivityRow {
    pub id: Uuid,
    pub admin_user_id: Uuid,
    pub admin_user_name: Option<String>,
    pub admin_email: Option<String>,
    pub action: String,
    pub entity_type: String,
    pub entity_id: Option<Uuid>,
    pub details: Option<Value>,
    pub created_at: DateTime<Utc>,
}

async fn count(pool: &PgPool, query: &str) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(query).fetch_one(pool).await
}

pub async fn get_statistics(pool: &PgPool) -> Result<AdminStatisticsDto, sqlx::Error> {
    Ok(AdminStatisticsDto {
        total_users: count(pool, "SELECT COUNT(*) FROM users").await?,
        active_users: count(pool, "SELECT COUNT(*) FROM users WHERE is_active = true").await?,
        disabled_users: count(pool, "SELECT COUNT(*) FROM users WHERE is_active = false").await?,
        admin_users: count(pool, "SELECT COUNT(*) FROM users WHERE role = 'admin'").await?,
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

pub async fn get_users(
    pool: &PgPool,
    search: Option<&str>,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<AdminUserRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT id, user_name, email, role,
               COALESCE((
                   SELECT SUM(earned_points)::INT
                   FROM user_task_progress
                   WHERE user_id = users.id
               ), 0) AS total_score,
               created_at, updated_at, avatar_url, is_active
        FROM users
        WHERE
            ($1::text IS NULL OR LOWER(user_name) LIKE '%' || LOWER($1) || '%'
             OR LOWER(email) LIKE '%' || LOWER($1) || '%')
            AND (
                $2::text IS NULL
                OR ($2 = 'active' AND is_active = true)
                OR ($2 = 'disabled' AND is_active = false)
                OR ($2 = 'admin' AND role = 'admin')
                OR ($2 = 'user' AND role = 'user')
            )
        ORDER BY created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(search)
    .bind(status)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_users(
    pool: &PgPool,
    search: Option<&str>,
    status: Option<&str>,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM users
        WHERE
            ($1::text IS NULL OR LOWER(user_name) LIKE '%' || LOWER($1) || '%'
             OR LOWER(email) LIKE '%' || LOWER($1) || '%')
            AND (
                $2::text IS NULL
                OR ($2 = 'active' AND is_active = true)
                OR ($2 = 'disabled' AND is_active = false)
                OR ($2 = 'admin' AND role = 'admin')
                OR ($2 = 'user' AND role = 'user')
            )
        "#,
    )
    .bind(search)
    .bind(status)
    .fetch_one(pool)
    .await
}

pub async fn get_user_by_id(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<AdminUserRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminUserRow>(
        r#"
        SELECT id, user_name, email, role,
               COALESCE((
                   SELECT SUM(earned_points)::INT
                   FROM user_task_progress
                   WHERE user_id = users.id
               ), 0) AS total_score,
               created_at, updated_at, avatar_url, is_active
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(pool)
    .await
}

pub async fn get_user_activity_summary(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<AdminUserActivitySummaryRow, sqlx::Error> {
    sqlx::query_as::<_, AdminUserActivitySummaryRow>(
        r#"
        SELECT
            (
                SELECT COUNT(DISTINCT sec.course_id)
                FROM user_task_progress p
                JOIN tasks t ON t.id = p.task_id
                JOIN sections sec ON sec.id = t.section_id
                WHERE p.user_id = $1
            ) AS courses_with_progress,
            (
                SELECT COUNT(*)
                FROM user_task_progress
                WHERE user_id = $1 AND status IN ('IN_PROGRESS', 'COMPLETED')
            ) AS started_tasks,
            (
                SELECT COUNT(*)
                FROM user_task_progress
                WHERE user_id = $1 AND status = 'COMPLETED'
            ) AS completed_tasks,
            (
                SELECT COUNT(*)
                FROM user_flags
                WHERE user_id = $1
            ) AS solved_flags,
            (
                SELECT COUNT(*)
                FROM environments
                WHERE user_id = $1 AND status IN ('Building', 'Running', 'Stopping')
            ) AS active_labs
        "#,
    )
    .bind(user_id)
    .fetch_one(pool)
    .await
}

pub async fn get_user_recent_labs(
    pool: &PgPool,
    user_id: Uuid,
    limit: i64,
) -> Result<Vec<AdminLabRow>, sqlx::Error> {
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
        WHERE e.user_id = $1
        ORDER BY e.created_at DESC
        LIMIT $2
        "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn update_user_status(
    pool: &PgPool,
    user_id: Uuid,
    is_active: bool,
) -> Result<Option<AdminUserRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminUserRow>(
        r#"
        UPDATE users
        SET is_active = $2, updated_at = now()
        WHERE id = $1
        RETURNING id, user_name, email, role,
                  COALESCE((
                      SELECT SUM(earned_points)::INT
                      FROM user_task_progress
                      WHERE user_id = users.id
                  ), 0) AS total_score,
                  created_at, updated_at, avatar_url, is_active
        "#,
    )
    .bind(user_id)
    .bind(is_active)
    .fetch_optional(pool)
    .await
}

pub async fn update_user_role(
    pool: &PgPool,
    user_id: Uuid,
    role: &str,
) -> Result<Option<AdminUserRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminUserRow>(
        r#"
        UPDATE users
        SET role = $2, updated_at = now()
        WHERE id = $1
        RETURNING id, user_name, email, role,
                  COALESCE((
                      SELECT SUM(earned_points)::INT
                      FROM user_task_progress
                      WHERE user_id = users.id
                  ), 0) AS total_score,
                  created_at, updated_at, avatar_url, is_active
        "#,
    )
    .bind(user_id)
    .bind(role)
    .fetch_optional(pool)
    .await
}

pub async fn update_user_password(
    pool: &PgPool,
    user_id: Uuid,
    password_hash: &str,
) -> Result<bool, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE users
        SET password_hash = $2, updated_at = now()
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .bind(password_hash)
    .execute(pool)
    .await?;

    Ok(result.rows_affected() == 1)
}

pub async fn active_admin_count(pool: &PgPool) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM users WHERE role = 'admin' AND is_active = true",
    )
    .fetch_one(pool)
    .await
}

pub async fn get_labs(
    pool: &PgPool,
    search: Option<&str>,
    status: Option<&str>,
    limit: i64,
    offset: i64,
) -> Result<Vec<AdminLabRow>, sqlx::Error> {
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
        WHERE
            ($1::text IS NULL OR LOWER(u.user_name) LIKE '%' || LOWER($1) || '%'
             OR LOWER(u.email) LIKE '%' || LOWER($1) || '%'
             OR LOWER(s.title) LIKE '%' || LOWER($1) || '%')
            AND ($2::text IS NULL OR e.status = $2)
        ORDER BY e.created_at DESC
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(search)
    .bind(status)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_labs(
    pool: &PgPool,
    search: Option<&str>,
    status: Option<&str>,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM environments e
        JOIN users u ON u.id = e.user_id
        JOIN scenarios s ON s.id = e.scenario_id
        WHERE
            ($1::text IS NULL OR LOWER(u.user_name) LIKE '%' || LOWER($1) || '%'
             OR LOWER(u.email) LIKE '%' || LOWER($1) || '%'
             OR LOWER(s.title) LIKE '%' || LOWER($1) || '%')
            AND ($2::text IS NULL OR e.status = $2)
        "#,
    )
    .bind(search)
    .bind(status)
    .fetch_one(pool)
    .await
}

pub async fn get_flags(
    pool: &PgPool,
    search: Option<&str>,
    scenario_id: Option<Uuid>,
    limit: i64,
    offset: i64,
) -> Result<Vec<AdminFlagRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminFlagRow>(
        r#"
        SELECT f.id, f.scenario_id, s.title AS scenario_title,
               f.flag_value, f.points
        FROM flags f
        LEFT JOIN scenarios s ON s.id = f.scenario_id
        WHERE
            ($1::text IS NULL OR LOWER(s.title) LIKE '%' || LOWER($1) || '%'
             OR LOWER(f.flag_value) LIKE '%' || LOWER($1) || '%')
            AND ($2::uuid IS NULL OR f.scenario_id = $2)
        ORDER BY s.title NULLS LAST, f.id
        LIMIT $3 OFFSET $4
        "#,
    )
    .bind(search)
    .bind(scenario_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(pool)
    .await
}

pub async fn count_flags(
    pool: &PgPool,
    search: Option<&str>,
    scenario_id: Option<Uuid>,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM flags f
        LEFT JOIN scenarios s ON s.id = f.scenario_id
        WHERE
            ($1::text IS NULL OR LOWER(s.title) LIKE '%' || LOWER($1) || '%'
             OR LOWER(f.flag_value) LIKE '%' || LOWER($1) || '%')
            AND ($2::uuid IS NULL OR f.scenario_id = $2)
        "#,
    )
    .bind(search)
    .bind(scenario_id)
    .fetch_one(pool)
    .await
}

pub async fn get_flag_by_id(
    pool: &PgPool,
    flag_id: Uuid,
) -> Result<Option<AdminFlagRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminFlagRow>(
        r#"
        SELECT f.id, f.scenario_id, s.title AS scenario_title,
               f.flag_value, f.points
        FROM flags f
        LEFT JOIN scenarios s ON s.id = f.scenario_id
        WHERE f.id = $1
        "#,
    )
    .bind(flag_id)
    .fetch_optional(pool)
    .await
}

pub async fn flag_exists_for_scenario(
    pool: &PgPool,
    scenario_id: Uuid,
    flag_value: &str,
    except_flag_id: Option<Uuid>,
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar::<_, bool>(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM flags
            WHERE scenario_id = $1
              AND flag_value = $2
              AND ($3::uuid IS NULL OR id <> $3)
        )
        "#,
    )
    .bind(scenario_id)
    .bind(flag_value)
    .bind(except_flag_id)
    .fetch_one(pool)
    .await
}

pub async fn create_flag(
    pool: &PgPool,
    scenario_id: Uuid,
    flag_value: &str,
    points: i32,
) -> Result<AdminFlagRow, sqlx::Error> {
    sqlx::query_as::<_, AdminFlagRow>(
        r#"
        INSERT INTO flags (scenario_id, flag_value, points)
        VALUES ($1, $2, $3)
        RETURNING
            id,
            scenario_id,
            (SELECT title FROM scenarios WHERE id = $1) AS scenario_title,
            flag_value,
            points
        "#,
    )
    .bind(scenario_id)
    .bind(flag_value)
    .bind(points)
    .fetch_one(pool)
    .await
}

pub async fn update_flag(
    pool: &PgPool,
    flag_id: Uuid,
    scenario_id: Uuid,
    flag_value: &str,
    points: i32,
) -> Result<Option<AdminFlagRow>, sqlx::Error> {
    sqlx::query_as::<_, AdminFlagRow>(
        r#"
        UPDATE flags
        SET scenario_id = $2, flag_value = $3, points = $4
        WHERE id = $1
        RETURNING
            id,
            scenario_id,
            (SELECT title FROM scenarios WHERE id = $2) AS scenario_title,
            flag_value,
            points
        "#,
    )
    .bind(flag_id)
    .bind(scenario_id)
    .bind(flag_value)
    .bind(points)
    .fetch_optional(pool)
    .await
}

pub async fn solved_flag_count(pool: &PgPool, flag_id: Uuid) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM user_flags WHERE flag_id = $1")
        .bind(flag_id)
        .fetch_one(pool)
        .await
}

pub async fn delete_flag(pool: &PgPool, flag_id: Uuid) -> Result<bool, sqlx::Error> {
    let result = sqlx::query("DELETE FROM flags WHERE id = $1")
        .bind(flag_id)
        .execute(pool)
        .await?;

    Ok(result.rows_affected() == 1)
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

pub async fn get_scenario_by_id(
    pool: &PgPool,
    scenario_id: Uuid,
) -> Result<Option<Scenario>, sqlx::Error> {
    sqlx::query_as::<_, Scenario>(
        r#"
        SELECT id, title, difficulty, description, vm_template_name,
               estimated_time_minutes, max_score, is_active
        FROM scenarios
        WHERE id = $1
        "#,
    )
    .bind(scenario_id)
    .fetch_optional(pool)
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

pub async fn insert_activity(
    pool: &PgPool,
    admin_user_id: Uuid,
    action: &str,
    entity_type: &str,
    entity_id: Option<Uuid>,
    details: Option<Value>,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO admin_activity_logs
            (admin_user_id, action, entity_type, entity_id, details)
        VALUES ($1, $2, $3, $4, $5)
        "#,
    )
    .bind(admin_user_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(details)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn get_activity(
    pool: &PgPool,
    admin_user_id: Option<Uuid>,
    action: Option<&str>,
    entity_type: Option<&str>,
    order: &str,
    limit: i64,
    offset: i64,
) -> Result<Vec<AdminActivityRow>, sqlx::Error> {
    let order_sql = if order == "asc" { "ASC" } else { "DESC" };
    let sql = format!(
        r#"
        SELECT l.id, l.admin_user_id, u.user_name AS admin_user_name,
               u.email AS admin_email, l.action, l.entity_type,
               l.entity_id, l.details, l.created_at
        FROM admin_activity_logs l
        LEFT JOIN users u ON u.id = l.admin_user_id
        WHERE ($1::uuid IS NULL OR l.admin_user_id = $1)
          AND ($2::text IS NULL OR l.action = $2)
          AND ($3::text IS NULL OR l.entity_type = $3)
        ORDER BY l.created_at {order_sql}
        LIMIT $4 OFFSET $5
        "#
    );

    sqlx::query_as::<_, AdminActivityRow>(&sql)
        .bind(admin_user_id)
        .bind(action)
        .bind(entity_type)
        .bind(limit)
        .bind(offset)
        .fetch_all(pool)
        .await
}

pub async fn count_activity(
    pool: &PgPool,
    admin_user_id: Option<Uuid>,
    action: Option<&str>,
    entity_type: Option<&str>,
) -> Result<i64, sqlx::Error> {
    sqlx::query_scalar::<_, i64>(
        r#"
        SELECT COUNT(*)
        FROM admin_activity_logs
        WHERE ($1::uuid IS NULL OR admin_user_id = $1)
          AND ($2::text IS NULL OR action = $2)
          AND ($3::text IS NULL OR entity_type = $3)
        "#,
    )
    .bind(admin_user_id)
    .bind(action)
    .bind(entity_type)
    .fetch_one(pool)
    .await
}
