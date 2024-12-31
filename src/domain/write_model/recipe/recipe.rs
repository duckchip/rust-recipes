use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Recipe {
    pub id: Option<String>,
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: String,
}