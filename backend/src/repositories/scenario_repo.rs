use sqlx::PgPool;
use uuid::Uuid;

use crate::models::entities::Scenario;

pub async fn find_active_scenario_by_id(
    pool: &PgPool,
    scenario_id: Uuid,
) -> Result<Option<Scenario>, sqlx::Error> {
    sqlx::query_as!(
        Scenario,
        r#"
        SELECT
            id,
            title,
            difficulty,
            description,
            vm_template_name,
            estimated_time_minutes,
            max_score,
            is_active
        FROM scenarios
        WHERE id = $1
          AND is_active = true
        "#,
        scenario_id
    )
    .fetch_optional(pool)
    .await
}
