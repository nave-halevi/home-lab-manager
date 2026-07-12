use sqlx::PgPool;
use uuid::Uuid;

pub async fn mark_task_completed(
    pool: &PgPool,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO user_task_progress (
            user_id,
            task_id,
            status,
            started_at,
            completed_at
        )
        VALUES (
            $1,
            $2,
            'COMPLETED',
            now(),
            now()
        )
        ON CONFLICT (user_id, task_id)
        DO UPDATE SET
            status = 'COMPLETED',
            started_at = COALESCE(
                user_task_progress.started_at,
                now()
            ),
            completed_at = COALESCE(
                user_task_progress.completed_at,
                now()
            )
        "#,
        user_id,
        task_id
    )
    .execute(pool)
    .await?;

    Ok(())
}