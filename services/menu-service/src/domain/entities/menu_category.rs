use azure_data_cosmos::CosmosEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::Component;

use crate::{app, domain};

use super::MenuItem;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct MenuCategory {
    pub id: String,
    pub category: String,
    pub description: String,
    pub menu_items: Vec<MenuItem>,
    pub created_by: Option<String>,
    pub created_on: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    pub modfied_on: Option<DateTime<Utc>>,
}

impl CosmosEntity for MenuCategory {
  type Entity = String;

  fn partition_key(&self) -> Self::Entity {
      self.id.clone()
  }
}

impl From<app::MenuCategoryDto> for MenuCategory {
    fn from(menu_category: app::MenuCategoryDto) -> Self {
        let mut menuitems: Vec<domain::MenuItem> = Vec::new();
        for mi in menu_category.menu_items {
            let mut ingredients: Vec<domain::Ingredient> = Vec::new();
            for ig in mi.ingredients {
                ingredients.push(domain::Ingredient {
                    id: ig.id,
                    category: ig.category,
                    name: ig.name,
                    quantity: ig.quantity,
                    uom: ig.uom,
                    created_by: None,
                    created_on: None,
                    modified_by: None,
                    modfied_on: None,
                });
            }
            menuitems.push(domain::MenuItem {
                id: mi.id,
                name: mi.name,
                price: mi.price,
                ingredients,
                status: mi.status,
                created_by: None,
                created_on: None,
                modified_by: None,
                modfied_on: None,
            });
        }

        Self {
            id: menu_category.id,
            category: menu_category.category,
            description: menu_category.description,
            menu_items: menuitems,
            created_by: None,
            created_on: None,
            modified_by: None,
            modfied_on: None,
        }
    }
}
