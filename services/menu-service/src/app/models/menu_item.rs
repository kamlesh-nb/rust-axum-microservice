use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

 

use super::IngredientDto;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct MenuItemDto{
  pub id: String,
  pub name: String,
  pub price: f32,
  pub ingredients: Vec<IngredientDto>,
  pub status: i8
}