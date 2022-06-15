use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

use crate::domain::MenuCategory;

use super::Ingredient;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct MenuItem{
  pub id: String,
  pub name: String,
  pub price: f32,
  pub ingredients: Vec<Ingredient>,
  pub status: i8
}