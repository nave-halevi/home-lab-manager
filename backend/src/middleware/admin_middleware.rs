use axum::{
    extract::Request,
    middleware::Next,
    response::Response,
    http::StatusCode,
};

use crate::models::user::{Claims, Role};

pub async fn admin_middleware(
    req: Request,
    next: Next,
) -> Result<Response, StatusCode> {

    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if claims.role != Role::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}