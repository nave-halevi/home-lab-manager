use sqlx::PgPool;
use uuid::Uuid;

#[derive(Debug)]
pub struct DashboardUserRow {
    pub user_name: String,
    pub total_score: i32,
}

#[derive(Debug)]
pub struct AvailableCourseRow {
    pub course_id: Uuid,
    pub course_title: String,
    pub course_description: Option<String>,
    pub difficulty: Option<String>,
    pub total_tasks: i64,
    pub total_points: i64,
}

#[derive(Debug)]
pub struct DashboardCourseDetailsRow {
    pub course_id: Uuid,
    pub course_title: String,
    pub course_description: Option<String>,
    pub difficulty: Option<String>,
}

pub async fn get_dashboard_user(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Option<DashboardUserRow>, sqlx::Error> {
    sqlx::query_as!(
        DashboardUserRow,
        r#"
        SELECT
            user_name,
            total_score
        FROM users
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_started_course_ids(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<Uuid>, sqlx::Error> {
    sqlx::query_scalar!(
        r#"
        SELECT DISTINCT c.id
        FROM courses c

        INNER JOIN sections s
            ON s.course_id = c.id

        INNER JOIN tasks t
            ON t.section_id = s.id

        INNER JOIN user_task_progress utp
            ON utp.task_id = t.id

        WHERE utp.user_id = $1
          AND c.is_published = true

        ORDER BY c.id
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_available_courses(
    pool: &PgPool,
    user_id: Uuid,
) -> Result<Vec<AvailableCourseRow>, sqlx::Error> {
    sqlx::query_as!(
        AvailableCourseRow,
        r#"
        SELECT
            c.id AS course_id,
            c.title AS course_title,
            c.description AS course_description,
            c.difficulty,

            COUNT(t.id) AS "total_tasks!",
            COALESCE(
                SUM(t.points),
                0
            ) AS "total_points!"

        FROM courses c

        LEFT JOIN sections s
            ON s.course_id = c.id

        LEFT JOIN tasks t
            ON t.section_id = s.id

        WHERE c.is_published = true

          AND NOT EXISTS (
              SELECT 1
              FROM user_task_progress utp

              INNER JOIN tasks progress_task
                  ON progress_task.id = utp.task_id

              INNER JOIN sections progress_section
                  ON progress_section.id =
                     progress_task.section_id

              WHERE utp.user_id = $1
                AND progress_section.course_id = c.id
          )

        GROUP BY
            c.id,
            c.title,
            c.description,
            c.difficulty,
            c.created_at

        ORDER BY c.created_at DESC
        "#,
        user_id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_course_details(
    pool: &PgPool,
    course_id: Uuid,
) -> Result<Option<DashboardCourseDetailsRow>, sqlx::Error> {
    sqlx::query_as!(
        DashboardCourseDetailsRow,
        r#"
        SELECT
            id AS course_id,
            title AS course_title,
            description AS course_description,
            difficulty
        FROM courses
        WHERE id = $1
        "#,
        course_id
    )
    .fetch_optional(pool)
    .await
}
