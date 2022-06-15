use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};



use super::{MenuItemDto};

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct MenuCategoryDto {
  pub id: String,
  pub category: String,
  pub description: String,
  pub menu_items: Vec<MenuItemDto>
}