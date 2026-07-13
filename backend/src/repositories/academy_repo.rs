use sqlx::PgPool;
use uuid::Uuid;

use crate::models::entities::{course::Course, section::Section, task::Task};

use crate::models::dto::{
    course::UpdateCourseRequest,
    section::UpdateSectionRequest,
    task::{CreateTaskRequest, UpdateTaskRequest},
};

// =====================================================
// Courses
// =====================================================

pub async fn create_course(
    pool: &PgPool,
    title: &str,
    slug: &str,
    description: Option<&str>,
    difficulty: Option<&str>,
) -> Result<Course, sqlx::Error> {
    sqlx::query_as!(
        Course,
        r#"
        INSERT INTO courses
            (title, slug, description, difficulty, is_published)

        VALUES
            ($1, $2, $3, $4, false)

        RETURNING
            id,
            title,
            slug,
            description,
            difficulty,
            is_published,
            created_at
        "#,
        title,
        slug,
        description,
        difficulty
    )
    .fetch_one(pool)
    .await
}

pub async fn get_courses(pool: &PgPool) -> Result<Vec<Course>, sqlx::Error> {
    sqlx::query_as!(
        Course,
        r#"
        SELECT
            id,
            title,
            slug,
            description,
            difficulty,
            is_published,
            created_at

        FROM courses

        ORDER BY created_at DESC
        "#
    )
    .fetch_all(pool)
    .await
}

pub async fn get_course_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Course>, sqlx::Error> {
    sqlx::query_as!(
        Course,
        r#"
        SELECT
            id,
            title,
            slug,
            description,
            difficulty,
            is_published,
            created_at

        FROM courses

        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn update_course(
    pool: &PgPool,
    id: Uuid,
    req: UpdateCourseRequest,
) -> Result<Course, sqlx::Error> {
    sqlx::query_as!(
        Course,
        r#"
        UPDATE courses

        SET
            title = COALESCE($2, title),
            slug = COALESCE($3, slug),
            description = COALESCE($4, description),
            difficulty = COALESCE($5, difficulty),
            is_published = COALESCE($6, is_published)

        WHERE id = $1

        RETURNING
            id,
            title,
            slug,
            description,
            difficulty,
            is_published,
            created_at
        "#,
        id,
        req.title,
        req.slug,
        req.description,
        req.difficulty,
        req.is_published
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_course(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM courses
        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// =====================================================
// Course Full
// =====================================================

#[derive(Debug)]
pub struct CourseFullRow {
    pub section_id: Uuid,
    pub section_title: String,
    pub section_order: i32,

    pub task_id: Option<Uuid>,
    pub task_section_id: Option<Uuid>,
    pub task_title: Option<String>,
    pub task_content: Option<String>,
    pub task_type: Option<String>,
    pub task_order: Option<i32>,
    pub scenario_id: Option<Uuid>,
    pub points: Option<i32>,
}

pub async fn get_course_full_rows(
    pool: &PgPool,
    course_id: Uuid,
) -> Result<Vec<CourseFullRow>, sqlx::Error> {
    sqlx::query_as!(
        CourseFullRow,
        r#"
        SELECT
                s.id as section_id,
                s.title as section_title,
                s.order_index as section_order,
                t.id as task_id,
                t.section_id as task_section_id,
                t.title as task_title,
                t.content as task_content,
                t.task_type,
                t.order_index as task_order,
                t.points,
                t.scenario_id

            FROM sections s

            LEFT JOIN tasks t
            ON t.section_id = s.id

            WHERE s.course_id = $1

            ORDER BY s.order_index, t.order_index
        "#,
        course_id
    )
    .fetch_all(pool)
    .await
}

// =====================================================
// Sections
// =====================================================

pub async fn create_section(
    pool: &PgPool,
    course_id: Uuid,
    title: &str,
    description: Option<&str>,
    order_index: i32,
) -> Result<Section, sqlx::Error> {
    sqlx::query_as!(
        Section,
        r#"
        INSERT INTO sections
            (course_id, title, description, order_index)

        VALUES
            ($1,$2,$3,$4)

        RETURNING
            id,
            course_id,
            title,
            description,
            order_index
        "#,
        course_id,
        title,
        description,
        order_index
    )
    .fetch_one(pool)
    .await
}

pub async fn get_section_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Section>, sqlx::Error> {
    sqlx::query_as!(
        Section,
        r#"
        SELECT
            id,
            course_id,
            title,
            description,
            order_index

        FROM sections

        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_sections_by_course(
    pool: &PgPool,
    course_id: Uuid,
) -> Result<Vec<Section>, sqlx::Error> {
    sqlx::query_as!(
        Section,
        r#"
        SELECT
            id,
            course_id,
            title,
            description,
            order_index

        FROM sections

        WHERE course_id = $1

        ORDER BY order_index
        "#,
        course_id
    )
    .fetch_all(pool)
    .await
}

pub async fn update_section(
    pool: &PgPool,
    id: Uuid,
    req: UpdateSectionRequest,
) -> Result<Section, sqlx::Error> {
    sqlx::query_as!(
        Section,
        r#"
        UPDATE sections

        SET
            title = COALESCE($2,title),
            description = COALESCE($3,description),
            order_index = COALESCE($4,order_index)

        WHERE id = $1

        RETURNING
            id,
            course_id,
            title,
            description,
            order_index
        "#,
        id,
        req.title,
        req.description,
        req.order_index
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_section(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM sections
        WHERE id=$1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}

// =====================================================
// Tasks
// =====================================================

pub async fn create_task(pool: &PgPool, req: CreateTaskRequest) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        INSERT INTO tasks
            (
                section_id,
                scenario_id,
                title,
                content,
                task_type,
                order_index,
                points
            )

        VALUES
            ($1,$2,$3,$4,$5,$6,COALESCE($7,10))


        RETURNING
            id,
            section_id,
            scenario_id,
            title,
            content,
            task_type,
            order_index,
            points
        "#,
        req.section_id,
        req.scenario_id,
        req.title,
        req.content,
        req.task_type,
        req.order_index,
        req.points
    )
    .fetch_one(pool)
    .await
}

pub async fn get_task_by_id(pool: &PgPool, id: Uuid) -> Result<Option<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        SELECT
            id,
            section_id,
            scenario_id,
            title,
            content,
            task_type,
            order_index,
            points

        FROM tasks

        WHERE id = $1
        "#,
        id
    )
    .fetch_optional(pool)
    .await
}

pub async fn get_tasks_by_section(
    pool: &PgPool,
    section_id: Uuid,
) -> Result<Vec<Task>, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        SELECT
            id,
            section_id,
            scenario_id,
            title,
            content,
            task_type,
            order_index,
            points

        FROM tasks

        WHERE section_id = $1

        ORDER BY order_index
        "#,
        section_id
    )
    .fetch_all(pool)
    .await
}

pub async fn update_task(
    pool: &PgPool,
    id: Uuid,
    req: UpdateTaskRequest,
) -> Result<Task, sqlx::Error> {
    sqlx::query_as!(
        Task,
        r#"
        UPDATE tasks

        SET

            title = COALESCE($2,title),

            content = COALESCE($3,content),

            task_type = COALESCE($4,task_type),

            scenario_id = COALESCE($5,scenario_id),

            order_index = COALESCE($6,order_index),

            points = COALESCE($7,points)


        WHERE id = $1


        RETURNING

            id,
            section_id,
            scenario_id,
            title,
            content,
            task_type,
            order_index,
            points
        "#,
        id,
        req.title,
        req.content,
        req.task_type,
        req.scenario_id,
        req.order_index,
        req.points
    )
    .fetch_one(pool)
    .await
}

pub async fn delete_task(pool: &PgPool, id: Uuid) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        DELETE FROM tasks

        WHERE id = $1
        "#,
        id
    )
    .execute(pool)
    .await?;

    Ok(())
}
