use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

use super::Ingredient;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct MenuItem{
  pub id: String,
  pub name: String,
  pub price: f32,
  pub ingredients: Vec<Ingredient>,
  pub status: i8,
  pub created_by: Option<String>,
  pub created_on: Option<DateTime<Utc>>,
  pub modified_by: Option<String>,
  pub modfied_on: Option<DateTime<Utc>>,
}