use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

pub mod users;
pub mod products;
pub mod orders;

pub async fn create_db_pool() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to create database pool")
}
