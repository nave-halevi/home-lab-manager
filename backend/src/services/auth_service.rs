use sqlx::PgPool;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::Utc;
use jsonwebtoken::{encode, decode,Header, Validation,EncodingKey, DecodingKey};

use crate::repositories::user_repo;
use crate::models::user::{RegisterRequest, LoginRequest, Role, Claims};


pub async fn register(
    pool: &PgPool,
    req: RegisterRequest,
) -> Result<crate::models::user::User, String> {

    let hashed = hash(&req.password, DEFAULT_COST)
        .map_err(|e| e.to_string())?;

    let user = user_repo::create_user(
        pool,
        &req.user_name,
        &req.email,
        &hashed,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(user)
}


pub async fn login(
    pool: &PgPool,
    req: LoginRequest,
) -> Result<String, String> {

    let user = user_repo::get_user_by_email(pool, &req.email)
        .await
        .map_err(|e| e.to_string())?
        .ok_or("Invalid credentials")?;

    let valid = verify(&req.password, &user.password_hash)
        .map_err(|e| e.to_string())?;

    if !valid {
        return Err("Invalid credentials".to_string());
    }

    let token = generate_token(&user.id.to_string(), user.role)?;
    Ok(token)
}


pub fn generate_token(user_id: &str, role: Role) -> Result<String, String> {

    let claims = Claims {
        sub: user_id.to_string(),
        exp: (Utc::now().timestamp() + 86400) as usize, 
        role,

    };

    let secret = std::env::var("JWT_SECRET")
        .map_err(|e| e.to_string())?;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|e| e.to_string())
}


pub fn verify_token(token: &str) -> Result<Claims, String> {

    let secret = std::env::var("JWT_SECRET")
        .map_err(|e| e.to_string())?;

    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    )
    .map(|data| data.claims)
    .map_err(|e| e.to_string())
}