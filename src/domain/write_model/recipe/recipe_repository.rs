use crate::models::recipe::Recipe;
use crate::db::get_db_collection;

pub struct RecipeRepository;

impl RecipeRepository {
    pub async fn save(recipe: Recipe) -> Result<(), Box<dyn std::error::Error>> {
        let collection = get_db_collection().await?;
        collection.insert_one(recipe, None).await?;
        Ok(())
    }

    pub async fn update_recipe(recipe: Recipe) -> Result<(), Box<dyn std::error::Error>> {
        let collection = get_db_collection().await?;
        let filter = doc! { "id": recipe.id.clone() };
        let update = doc! {
            "$set": {
                "name": recipe.name,
                "ingredients": recipe.ingredients,
                "instructions": recipe.instructions
            }
        };
        collection.update_one(filter, update, None).await?;
        Ok(())
    }

    pub async fn delete_recipe(id: &str) -> Result<(), Box<dyn std::error::Error>> {
        let collection = get_db_collection().await?;
        let filter = doc! { "id": id };
        collection.delete_one(filter, None).await?;
        Ok(())
    }
}