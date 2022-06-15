use serde::{Deserialize, Serialize};
use utoipa::Component;


#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct IngredientDto{
  pub id: String,
  pub category: String,
  pub name: String,
  pub quantity: f32,
  pub uom: String,
}