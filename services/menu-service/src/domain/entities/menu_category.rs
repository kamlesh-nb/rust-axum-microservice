use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

use crate::domain::MenuCategory;

use super::MenuItem;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct MenuCategory{
  pub id: String,
  pub category: String,
  pub description: String,
  pub menu_items: Vec<MenuItem>
}