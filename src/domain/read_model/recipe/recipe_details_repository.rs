use serde::{Deserialize, Serialize};
use sqlx::{PgPool};
use sqlx::{Error};

pub struct RecipeDetailsRepository;

impl RecipeDetailsRepository {
    // Get a recipe by ID
    pub async fn get_recipe_by_id(pool: &PgPool, id: i32) -> Result<RecipeDetails, Error> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, ingredients, instructions
            FROM recipes
            WHERE id = $1
            "#,
            id
        )
        .fetch_one(pool)
        .await?;

        let recipe = RecipeDetails {
            id: row.id,
            name: row.name.unwrap(),
            ingredients: row.ingredients.unwrap(),
            instructions: row.instructions.unwrap(),
        };

        Ok(recipe)
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct RecipeDetails {
    pub id: i32,
    pub name: String,
    pub ingredients: serde_json::Value,
    pub instructions: String,
}