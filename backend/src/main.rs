use axum::{Router};
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;

mod utils;
mod db;
mod handlers;
mod models;
mod routes;
mod services;


#[tokio::main]
async fn main() {

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL environment variable is missing!");
   
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await.expect("Failed to connect to the database");
    
    println!("✅ Successfully connected to PostgreSQL!");

    let app = Router::new()
        .route("/", axum::routing::get(|| async {"Hello from the Rust Backend!"}))
        .nest("/api/users", crate::routes::user_routes::build_user_routes().await)
        .with_state(pool);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await.unwrap();

    println!("🚀 Server is running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();

}