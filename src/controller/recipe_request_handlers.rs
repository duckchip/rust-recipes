use std::collections::HashMap;

use axum::{Json, routing::get, Router};
use axum::extract::{Path, Extension};
use sqlx::PgPool;

use crate::domain::read_model::overview::{pagination_from_request, sorting_from_request, Overview, OverviewRequest};
use crate::domain::read_model::recipe::recipe_overview_repository::{RecipeOverviewItem, RecipeOverviewRepository};
use crate::domain::read_model::recipe::recipe_details_repository::{RecipeDetails, RecipeDetailsRepository};


pub fn create_routes() -> Router {
    Router::new()
        .route("/:id", get(get_recipe))
        .route("/overview", get(get_overview))
}

pub async fn get_recipe(
    Path(id): Path<i32>, 
    Extension(pool): Extension<PgPool>,
)-> Json<Option<RecipeDetails>> {
    match RecipeDetailsRepository::get_recipe_by_id(&pool, id).await {
        Ok(recipe) => Json(Some(recipe)),
        Err(_) => Json(None),
    }
}

pub async fn get_overview(
    Path(params): Path<HashMap<String, String>>,
    Extension(pool): Extension<PgPool>,
) -> Json<Overview<RecipeOverviewItem>> {
 
    match RecipeOverviewRepository::find_one_by(
        &pool, &OverviewRequest {
           pagination: pagination_from_request(&params),
           sorting: sorting_from_request(&params)
        }).await {
        Ok(recipes) => Json(recipes),
        Err(_) => {
            eprintln!("Error fetching overview");
            Json(Overview {
                total_items: 0,
                total_pages: 0,
                current_page: 0,
                page_size: 0,
                items: Vec::new(),
            })
        }
    }
}

