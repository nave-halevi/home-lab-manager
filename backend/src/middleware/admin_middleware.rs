use axum::{extract::Request, http::StatusCode, middleware::Next, response::Response};

use crate::models::user::{Claims, Role};

pub async fn admin_middleware(req: Request, next: Next) -> Result<Response, StatusCode> {
    let claims = req
        .extensions()
        .get::<Claims>()
        .ok_or(StatusCode::UNAUTHORIZED)?;

    if claims.role != Role::Admin {
        return Err(StatusCode::FORBIDDEN);
    }

    Ok(next.run(req).await)
}
