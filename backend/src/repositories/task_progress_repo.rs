use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct TaskProgressRow {
    pub task_id: Uuid,
    pub task_title: String,
    pub task_type: String,
    pub task_order_index: i32,

    pub section_id: Uuid,
    pub section_title: String,
    pub section_order_index: i32,

    pub points: i32,
    pub earned_points: i32,
    pub status: String,

    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
}

pub async fn mark_task_completed(
    pool: &PgPool,
    user_id: Uuid,
    task_id: Uuid,
) -> Result<i32, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO user_task_progress (
            user_id,
            task_id,
            status,
            started_at,
            completed_at,
            earned_points
        )
        SELECT
            $1,
            t.id,
            'COMPLETED',
            now(),
            now(),
            t.points
        FROM tasks t
        WHERE t.id = $2
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
            ),
            earned_points = CASE
                WHEN user_task_progress.status = 'COMPLETED'
                    THEN user_task_progress.earned_points
                ELSE EXCLUDED.earned_points
            END
        RETURNING earned_points
        "#,
        user_id,
        task_id
    )
    .fetch_one(pool)
    .await?;

    Ok(row.earned_points)
}

pub async fn get_course_progress(
    pool: &PgPool,
    user_id: Uuid,
    course_id: Uuid,
) -> Result<Vec<TaskProgressRow>, sqlx::Error> {
    let progress = sqlx::query_as!(
        TaskProgressRow,
        r#"
        SELECT
            t.id AS task_id,
            t.title AS task_title,
            t.task_type,
            t.order_index AS task_order_index,

            s.id AS section_id,
            s.title AS section_title,
            s.order_index AS section_order_index,

            t.points,
            COALESCE(
                utp.earned_points,
                0
            ) AS "earned_points!",

            COALESCE(
                utp.status,
                'NOT_STARTED'
            ) AS "status!",

            utp.started_at,
            utp.completed_at

        FROM tasks t

        INNER JOIN sections s
            ON s.id = t.section_id

        LEFT JOIN user_task_progress utp
            ON utp.task_id = t.id
            AND utp.user_id = $1

        WHERE s.course_id = $2

        ORDER BY
            s.order_index ASC,
            t.order_index ASC
        "#,
        user_id,
        course_id
    )
    .fetch_all(pool)
    .await?;

    Ok(progress)
}

pub async fn get_course_id_by_task_id(
    pool: &PgPool,
    task_id: Uuid,
) -> Result<Option<Uuid>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT s.course_id
        FROM tasks t
        INNER JOIN sections s
            ON s.id = t.section_id
        WHERE t.id = $1
        "#,
        task_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn mark_task_in_progress(
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
            completed_at,
            earned_points
        )
        VALUES (
            $1,
            $2,
            'IN_PROGRESS',
            now(),
            NULL,
            0
        )
        ON CONFLICT (user_id, task_id)
        DO UPDATE SET
            status = CASE
                WHEN user_task_progress.status = 'COMPLETED'
                    THEN 'COMPLETED'
                ELSE 'IN_PROGRESS'
            END,
            started_at = COALESCE(
                user_task_progress.started_at,
                now()
            ),
            completed_at = CASE
                WHEN user_task_progress.status = 'COMPLETED'
                    THEN user_task_progress.completed_at
                ELSE NULL
            END
        "#,
        user_id,
        task_id
    )
    .execute(pool)
    .await?;

    Ok(())
}
