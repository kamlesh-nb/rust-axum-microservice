use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

use crate::domain::Rating;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct RatingDto {
  pub id: String,
  pub customer_id: String,
  pub menu_id: String,
  pub score: i16,
  pub remarks: String,
  pub date_recorded: DateTime<Utc>,
}

impl From<Rating> for RatingDto {
  fn from(entity: Rating) -> Self {
      Self {
          id: entity.id,
          customer_id: entity.customer_id,
          menu_id: entity.menu_id,
          score: entity.score,
          remarks: entity.remarks,
          date_recorded: entity.date_recorded,
      }
  }
}

