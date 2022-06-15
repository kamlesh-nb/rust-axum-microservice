use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct Ingredient{
  pub id: String,
  pub category: String,
  pub name: String,
  pub quantity: f32,
  pub uom: String,
  pub created_by: Option<String>,
  pub created_on: Option<DateTime<Utc>>,
  pub modified_by: Option<String>,
  pub modfied_on: Option<DateTime<Utc>>,
}