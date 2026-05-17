use chrono::{DateTime, Utc};
use uuid::Uuid;
use serde::{Serialize, Deserialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct User {
    pub id: Uuid,
    pub user_name: String,
    pub email: String,
    pub password_hash: String,    
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub user_name: String,
    pub email: String,
    pub password: String,
}