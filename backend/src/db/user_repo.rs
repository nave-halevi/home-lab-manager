use sqlx::PgPool;
use crate::models::user::User;

pub async fn create_user(
    pool: &PgPool,
    user_name: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error>{
    let user = sqlx::query_as!(
        User,
        r#"
        INSERT INTO users (user_name, email, password_hash)
        VALUES ($1, $2, $3)
        RETURNING *
        "#,
        user_name,
        email,
        password_hash
    ) 
    .fetch_one(pool)
    .await?;

    Ok(user)
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users
        "#
    )
    .fetch_all(pool)
    .await?;
    Ok(users)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User> ,sqlx::Error> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT * FROM users WHERE email=$1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;
    Ok(user)
}

