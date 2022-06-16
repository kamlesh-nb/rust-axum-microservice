use azure_data_cosmos::CosmosEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::Component;

use crate::{domain, app};

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct IngredientCategory {
    pub id: String,
    pub category: String,
    pub ingredients: Vec<super::Ingredient>,
    pub created_by: Option<String>,
    pub created_on: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    pub modfied_on: Option<DateTime<Utc>>,
}

impl CosmosEntity for IngredientCategory {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}


impl From<app::IngredientCategoryDto> for IngredientCategory {
  fn from(ingredient_category: app::IngredientCategoryDto) -> Self {
      let mut ingredients: Vec<domain::Ingredient> = Vec::new();
      for ig in ingredient_category.ingredients {
        ingredients.push(domain::Ingredient{
            id: ig.id,
            name: ig.name,
            uom: ig.uom,
        });
      }

      Self { 
        id: ingredient_category.id,
        category: ingredient_category.category,
        ingredients,
        created_by: None,
        created_on: None,
        modified_by: None,
        modfied_on: None, 
      }
  }
}