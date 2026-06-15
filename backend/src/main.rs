use axum::{Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tower_http::cors::{Any, CorsLayer};
use axum::http::Method;

mod utils;
mod db;
mod handlers;
mod models;
mod routes;
mod services;

#[tokio::main]
async fn main() {
    // 1. טעינת משתני סביבה מהקובץ .env
    dotenv().ok();

    // 2. חיבור למסד הנתונים
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable is missing!");
   
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.expect("Failed to connect to the database");
    
    println!("✅ Successfully connected to PostgreSQL!");

    // 3. הגדרת מדיניות CORS (מאפשר ל-Frontend ב-React לדבר עם השרת)
    let cors = CorsLayer::new()
        .allow_origin(Any) // מאפשר לכל דומיין לגשת (מעולה לפיתוח)
        .allow_methods([Method::GET, Method::POST, Method::DELETE])
        .allow_headers(Any);

    // 4. בניית הראוטר הראשי
    let app = Router::new()
        .nest("/api", routes::create_api_router())
        .layer(cors) // הוספת שכבת ה-CORS לראוטר
        .with_state(pool);

    // 5. האזנה לפורט 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();

    println!("🚀 Server is running on http://localhost:3000");

    // 6. הרצת השרת
    axum::serve(listener, app).await.unwrap();
}