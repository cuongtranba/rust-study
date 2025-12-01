use anyhow::Context;
use sqlx::postgres::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration; // Add this import

pub struct AppState {
    db: PgPool,
}

pub async fn create_pool(database_url: &str) -> Result<PgPool, anyhow::Error> {
    PgPoolOptions::new()
        .max_connections(20)
        .min_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .idle_timeout(Duration::from_secs(300))
        .max_lifetime(Duration::from_secs(1800))
        .connect(database_url)
        .await
        .context("Failed to create database pool")
}

fn main() {
    println!("Hello, world!");
}
