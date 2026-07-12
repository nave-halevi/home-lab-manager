use sqlx::PgPool;
use uuid::Uuid;

use crate::{
    models::{
        entities::Instance,
        status::InstanceStatus,
    },
};

pub async fn create_instance(
    pool: &PgPool,
    environment_id: Uuid,
    vm_name: &str,
    ssh_port: i32,
    is_entry_point: bool,
) -> Result<Instance, sqlx::Error> {

    let status = InstanceStatus::Starting.as_str();

    sqlx::query_as!(
        Instance,
        r#"
        INSERT INTO instances
        (
            environment_id,
            vm_name,
            ssh_port,
            is_entry_point,
            status
        )
        VALUES
        (
            $1,
            $2,
            $3,
            $4,
            $5
        )
        RETURNING
            id,
            environment_id,
            vm_name,
            is_entry_point,
            internal_ip,
            created_at,
            ssh_port,
            status,
            last_activity
        "#,
        environment_id,
        vm_name,
        ssh_port,
        is_entry_point,
        status
    )
    .fetch_one(pool)
    .await
}

pub async fn find_by_environment_id(
    pool: &PgPool,
    environment_id: Uuid,
) -> Result<Option<Instance>, sqlx::Error> {

    sqlx::query_as!(
        Instance,
        r#"
        SELECT
            id,
            environment_id,
            vm_name,
            is_entry_point,
            internal_ip,
            created_at,
            ssh_port,
            status,
            last_activity
        FROM instances
        WHERE environment_id = $1
        ORDER BY created_at DESC
        LIMIT 1
        "#,
        environment_id
    )
    .fetch_optional(pool)
    .await
}

pub async fn update_instance_status(
    pool: &PgPool,
    instance_id: Uuid,
    status: InstanceStatus,
) -> Result<Instance, sqlx::Error> {

    sqlx::query_as!(
        Instance,
        r#"
        UPDATE instances
        SET
            status = $2,
            last_activity = now()
        WHERE id = $1
        RETURNING
            id,
            environment_id,
            vm_name,
            is_entry_point,
            internal_ip,
            created_at,
            ssh_port,
            status,
            last_activity
        "#,
        instance_id,
        status.as_str()
    )
    .fetch_one(pool)
    .await
}

pub async fn update_instance_status_by_environment(
    pool: &PgPool,
    environment_id: Uuid,
    status: InstanceStatus,
) -> Result<(), sqlx::Error> {

    sqlx::query!(
        r#"
        UPDATE instances
        SET
            status = $2,
            last_activity = now()
        WHERE environment_id = $1
        "#,
        environment_id,
        status.as_str()
    )
    .execute(pool)
    .await?;

    Ok(())
}