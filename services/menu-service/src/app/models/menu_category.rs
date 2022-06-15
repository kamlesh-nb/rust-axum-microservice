use serde::{Deserialize, Serialize};
use utoipa::Component;


use crate::{domain, app};

use super::{MenuItemDto, IngredientDto};

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct MenuCategoryDto {
  pub id: String,
  pub category: String,
  pub description: String,
  pub menu_items: Vec<MenuItemDto>,
  pub processed_by: String,
}

impl From<domain::MenuCategory> for MenuCategoryDto {
  fn from(menu_category: domain::MenuCategory) -> Self {
      let mut menuitems: Vec<app::MenuItemDto> = Vec::new();
      for mi in menu_category.menu_items {
        let mut ingredients: Vec<IngredientDto> = Vec::new();
        for ig in mi.ingredients {
          ingredients.push(app::IngredientDto{
            id: ig.id,
            category: ig.category,
            name: ig.name,
            quantity: ig.quantity,
            uom: ig.uom,
        });
        }
        menuitems.push(app::MenuItemDto{
            id: mi.id,
            name: mi.name,
            price: mi.price,
            ingredients,
            status: mi.status,
        });
      }

      Self {
        id: menu_category.id,
        category: menu_category.category,
        description: menu_category.description,
        menu_items: menuitems,
        processed_by: Some(menu_category.created_by).unwrap().unwrap(),
    }
  }
}