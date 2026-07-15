use sqlx::{Error, PgPool};
use uuid::Uuid;

pub async fn get_env_details(pool: &PgPool, env_id: Uuid) -> Result<Option<(Uuid, Uuid)>, Error> {
    let record = sqlx::query!(
        "SELECT user_id, scenario_id FROM environments WHERE id = $1",
        env_id
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| (r.user_id, r.scenario_id)))
}

pub async fn get_flag(
    pool: &PgPool,
    scenario_id: Uuid,
    flag_value: &str,
) -> Result<Option<(Uuid, i32)>, Error> {
    let record = sqlx::query!(
        "SELECT id, points FROM flags WHERE scenario_id = $1 AND flag_value = $2",
        scenario_id,
        flag_value
    )
    .fetch_optional(pool)
    .await?;

    Ok(record.map(|r| (r.id, r.points)))
}

pub async fn submit_and_score(pool: &PgPool, user_id: Uuid, flag_id: Uuid) -> Result<(), Error> {
    sqlx::query!(
        "INSERT INTO user_flags (user_id, flag_id) VALUES ($1, $2)",
        user_id,
        flag_id
    )
    .execute(pool)
    .await?;
    Ok(())
}
