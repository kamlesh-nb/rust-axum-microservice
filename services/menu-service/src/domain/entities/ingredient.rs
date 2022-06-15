use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

use crate::domain::MenuCategory;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct Ingredient{
  pub id: String,
  pub category: String,
  pub name: String,
  pub quantity: f32,
  pub uom: String
}