use axum::{extract::Request, middleware::Next, response::Response};

use crate::{
    errors::AppError,
    models::user::{Claims, Role},
};

pub async fn admin_middleware(req: Request, next: Next) -> Result<Response, AppError> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or(AppError::Unauthorized)?;

    if claims.role != Role::Admin {
        return Err(AppError::Forbidden);
    }

    Ok(next.run(req).await)
}
