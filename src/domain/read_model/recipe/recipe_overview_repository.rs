use sqlx::{PgPool, Error};
use crate::domain::read_model::overview::{Overview, OverviewRequest};
use serde::{Deserialize, Serialize};

pub struct RecipeOverviewRepository;

impl RecipeOverviewRepository {
    pub async fn find_one_by(
        pool: &PgPool,
        request: &OverviewRequest,
    ) -> Result<Overview<RecipeOverviewItem>, Error> {
        // Calculate pagination parameters
        let offset = ((request.pagination.page - 1) * request.pagination.size) as i64;
        let size = request.pagination.size  as i64;
        // Get the total number of recipes
        let total_items: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM recipes")
            .fetch_one(pool)
            .await?;
    
        // Fetch the recipes for the current page
        let rows = sqlx::query!(
            r#"
            SELECT id, name, ingredients, instructions
            FROM recipes
            LIMIT $1 OFFSET $2
            "#,
            size,
            offset
        )
        .fetch_all(pool)
        .await?;
    
        // Map the results into RecipeItem structs
        let items: Vec<RecipeOverviewItem> = rows
            .into_iter()
            .map(|row| RecipeOverviewItem {
                id: row.id,
                name: row.name.unwrap(),
                ingredients: row.ingredients.unwrap(),
                instructions: row.instructions.unwrap(),
            })
            .collect();

        let size = request.pagination.size as i64;
    
        // Calculate total pages
        let total_pages = if total_items.0 % size == 0 {
            total_items.0 / size
        } else {
            total_items.0 / size + 1
        };
    
        // Create the overview struct
        let overview = Overview {
            total_items: total_items.0.try_into().unwrap(),
            page_size: request.pagination.size,
            current_page: request.pagination.page,
            total_pages: total_pages.try_into().unwrap(),
            items,
        };
    
        Ok(overview)
    }
}

// Specific recipe overview (simplified for listing)
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeOverviewItem {
    pub id: i32,
    pub name: String,
    pub ingredients: serde_json::Value,
    pub instructions: String,
}