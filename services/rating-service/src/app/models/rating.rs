use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

use crate::domain::Rating;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct RatingDto {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub address: String,
  pub city: String,
  pub state: String,
  pub phone: String,
  pub birthday: DateTime<Utc>,
  pub favourite_dishes: Vec<String>
}

impl From<Rating> for RatingDto {
  fn from(entity: Rating) -> Self {
      Self {
          id: entity.id,
          first_name: entity.first_name,
          last_name: entity.last_name,
          email: entity.email,
          address: entity.address,
          city: entity.city,
          state: entity.state,
          phone: entity.phone,
          birthday: entity.birthday,
          favourite_dishes: entity.favourite_dishes,
      }
  }
}

