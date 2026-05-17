use sqlx::PgPool;
use bcrypt::{hash, DEFAULT_COST};
use crate::models::user::{User, RegisterRequest};
use crate::db::user_repo;

pub fn encrypt_password(password: &str) -> Result<String, String> {
    hash(password, DEFAULT_COST).map_err(|e| e.to_string())
}

pub async fn register_new_user(
    pool: &PgPool,
    req: RegisterRequest,
) -> Result<User, String>{
    
    let hashed_password = encrypt_password(&req.password)?;

    let user = user_repo::create_user(
        pool,
        &req.user_name,
        &req.email,
        &hashed_password
    )
        .await
        .map_err(|e| e.to_string())?;
    Ok(user)
}

pub async fn fetch_all_users(pool: &PgPool) -> Result <Vec<User>, String> {
    let users  = user_repo::get_all_users(pool)
    .await
    .map_err(|e| e.to_string());

    Ok(users?)
}