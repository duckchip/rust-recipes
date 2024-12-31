pub mod recipe_request_handlers;

use axum::Router;

pub fn create_routes() -> Router {
    Router::new()
        .nest("/recipes", recipe_request_handlers::create_routes())  // Register recipe-related routes
}