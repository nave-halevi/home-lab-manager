use crate::repositories::user_repo;
use sqlx::PgPool;

pub async fn fetch_all_users(pool: &PgPool) -> Result<Vec<crate::models::user::User>, String> {
    user_repo::get_all_users(pool)
        .await
        .map_err(|e| e.to_string())
}
