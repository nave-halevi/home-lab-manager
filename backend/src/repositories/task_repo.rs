use sqlx::PgPool;
use uuid::Uuid;

pub async fn task_belongs_to_scenario(
    pool: &PgPool,
    task_id: Uuid,
    scenario_id: Uuid,
) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM tasks
            WHERE id = $1
              AND scenario_id = $2
              AND task_type = 'LAB'
        ) AS "exists!"
        "#,
        task_id,
        scenario_id
    )
    .fetch_one(pool)
    .await?;

    Ok(exists)
}
