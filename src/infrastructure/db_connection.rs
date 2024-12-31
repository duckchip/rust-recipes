use sqlx::{PgPool, postgres::PgPoolOptions};
use std::env;
use tokio::sync::OnceCell;
use dotenv::dotenv;

/// A global, lazy-loaded connection pool.
pub static DB_POOL: OnceCell<PgPool> = OnceCell::const_new();

/// Establishes a connection pool for the PostgreSQL database and stores it in a global static.
pub async fn init_db() -> Result<PgPool, sqlx::Error> {
    dotenv().ok(); // Load environment variables from `.env` file, if present

    // Get the database URL from the environment or from the `.env` file
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in the environment or .env file");

    // Create the database pool
    let pool = PgPoolOptions::new()
        .max_connections(5) // Set the maximum number of database connections
        .connect(&database_url) // Connect to the database
        .await?;

    // Store the pool in the global static variable
    DB_POOL.set(pool).expect("Failed to initialize the database pool");

    Ok(DB_POOL.get().unwrap().clone()) // Return a clone of the pool
}

/// Get a reference to the global database pool.
pub fn get_db_pool() -> PgPool {
    DB_POOL.get().expect("Database pool not initialized").clone()
}