use serde::{Deserialize, Serialize};
use utoipa::Component;

use crate::{domain, app};
 

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct IngredientCategoryDto {
    pub id: String,
    pub category: String,
    pub ingredients: Vec<super::IngredientDto>,
    pub processed_by: String,
}


impl From<domain::IngredientCategory> for IngredientCategoryDto {
  fn from(ingredient_category: domain::IngredientCategory) -> Self {
      let mut ingredients: Vec<app::IngredientDto> = Vec::new();
      for ig in ingredient_category.ingredients {
        ingredients.push(app::IngredientDto{
            id: ig.id,
            name: ig.name,
            uom: ig.uom,
        });
      }

      Self { 
        id: ingredient_category.id,
        category: ingredient_category.category,
        ingredients, 
        processed_by: Some(ingredient_category.created_by).unwrap().unwrap(),
      }
  }
}