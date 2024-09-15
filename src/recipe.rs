use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    pub id: u32,
    pub name: String,
    pub ingredients: Vec<String>,
    pub instructions: Vec<String>,
    pub servings: u32,
}

impl Recipe {
    pub fn new(id: u32, name: String, ingredients: Vec<String>, instructions: Vec<String>, servings: u32) -> Self {
        Recipe {
            id,
            name,
            ingredients,
            instructions,
            servings,
        }
    }
}