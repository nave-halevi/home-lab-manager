use axum::{Extension, extract::Request, http::header, middleware::Next, response::Response};
use sqlx::{FromRow, PgPool};
use uuid::Uuid;

use crate::{errors::AppError, models::user::Role, services::auth_service};

#[derive(Debug, FromRow)]
struct AuthUserRow {
    role: String,
    is_active: bool,
}

pub async fn auth_middleware(
    Extension(pool): Extension<PgPool>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    let auth_header = req
        .headers()
        .get(header::AUTHORIZATION)
        .and_then(|h| h.to_str().ok())
        .ok_or(AppError::Unauthorized)?;

    let token = auth_header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;

    let mut claims = auth_service::verify_token(token).map_err(|_| AppError::Unauthorized)?;
    let user_id: Uuid = claims.sub.parse().map_err(|_| AppError::Unauthorized)?;

    let current_user = sqlx::query_as::<_, AuthUserRow>(
        r#"
        SELECT role, is_active
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(user_id)
    .fetch_optional(&pool)
    .await?;

    let current_user = current_user.ok_or(AppError::Unauthorized)?;
    if !current_user.is_active {
        return Err(AppError::ForbiddenMessage(
            "This account has been disabled.".to_string(),
        ));
    }

    claims.role = if current_user.role == "admin" {
        Role::Admin
    } else {
        Role::User
    };
    req.extensions_mut().insert(claims);

    Ok(next.run(req).await)
}
