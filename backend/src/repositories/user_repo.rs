use crate::models::user::{Role, User};
use sqlx::PgPool;

pub async fn create_user(
    pool: &PgPool,
    user_name: &str,
    email: &str,
    password_hash: &str,
) -> Result<User, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        INSERT INTO users (user_name, email, password_hash, role)
        VALUES ($1, $2, $3, 'user')
        RETURNING id, user_name, email, password_hash, role, created_at, updated_at, total_score, avatar_url
        "#,
        user_name,
        email,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    let role = match row.role.as_str() {
        "admin" => Role::Admin,
        _ => Role::User,
    };

    let user = User {
        id: row.id,
        user_name: row.user_name,
        email: row.email,
        password_hash: row.password_hash,
        created_at: row.created_at,
        updated_at: row.updated_at,
        total_score: row.total_score,
        role,
        avatar_url: row.avatar_url,
    };

    Ok(user)
}

pub async fn get_all_users(pool: &PgPool) -> Result<Vec<User>, sqlx::Error> {
    let rows = sqlx::query!(
        r#"
        SELECT id, user_name, email, password_hash, role, created_at, updated_at, total_score, avatar_url
        FROM users
        "#
    )
    .fetch_all(pool)
    .await?;

    let users = rows
        .into_iter()
        .map(|row| {
            let role = match row.role.as_str() {
                "admin" => Role::Admin,
                _ => Role::User,
            };

            User {
                id: row.id,
                user_name: row.user_name,
                email: row.email,
                password_hash: row.password_hash,
                created_at: row.created_at,
                updated_at: row.updated_at,
                total_score: row.total_score,
                role,
                avatar_url: row.avatar_url,
            }
        })
        .collect();

    Ok(users)
}

pub async fn get_user_by_email(pool: &PgPool, email: &str) -> Result<Option<User>, sqlx::Error> {
    let row = sqlx::query!(
        r#"
        SELECT id, user_name, email, password_hash, role, created_at, updated_at, total_score, avatar_url
        FROM users
        WHERE email = $1
        "#,
        email
    )
    .fetch_optional(pool)
    .await?;

    let user = match row {
        Some(row) => {
            let role = match row.role.as_str() {
                "admin" => Role::Admin,
                _ => Role::User,
            };

            Some(User {
                id: row.id,
                user_name: row.user_name,
                email: row.email,
                password_hash: row.password_hash,
                created_at: row.created_at,
                updated_at: row.updated_at,
                total_score: row.total_score,
                role,
                avatar_url: row.avatar_url,
            })
        }
        None => None,
    };

    Ok(user)
}
