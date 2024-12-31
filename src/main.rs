use axum::{Extension, Router};
use std::net::SocketAddr;
use crate::infrastructure::db_connection::init_db;  // Import the database initialization function

mod infrastructure;
mod domain;
mod controller;

#[tokio::main]
async fn main() {
    // Initialize the database connection pool
    match init_db().await {
        Ok(pool) => {
            // Set the address to bind the server
            let addr = SocketAddr::from(([127, 0, 0, 1], 8080));

            // Build the application by using the routes from controller
            let app = Router::new()
                .nest("/", controller::create_routes())  // Register all routes defined in controller
                .layer(Extension(pool));  // Add the database pool as shared state to the app

            // Start the server using Axum's built-in server (which uses Hyper internally)
            println!("Server running at http://127.0.0.1:8080");
            if let Err(err) = axum_server::bind(addr).serve(app.into_make_service()).await {
                eprintln!("Server error: {}", err);
            }
        },
        Err(err) => {
            eprintln!("Error initializing database: {}", err);
        }
    }
}