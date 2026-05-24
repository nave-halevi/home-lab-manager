use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use crate::models::user::{User, RegisterRequest ,LoginRequest};
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


//  Function to handle user login and return a JWT token if successful .
pub async fn login_user(
    pool: &PgPool, 
    req: LoginRequest
) -> Result<String, String> {
    
    let user_option = user_repo::get_user_by_email(pool, &req.email)
    .await
    .map_err(|e| e.to_string())?;

    let user = match user_option {
        Some(u) => u,
        None => return Err("Incorrect email or password".to_string()),
    };

    let is_valid = verify( &req.password, &user.password_hash)
    .map_err(|e| e.to_string())?;

    if !is_valid {
        return Err("Incorrect email".to_string());
    }

    let token = crate::utils::jwt::generate_token(&user.id.to_string())?;
    Ok(token)
    
}