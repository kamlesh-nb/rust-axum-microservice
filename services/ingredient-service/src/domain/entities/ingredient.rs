use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Ingredient {
  pub id: i32,
  pub name: String,
  pub uom: String,
}