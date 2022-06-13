use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct CustomerDto {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub address: String,
  pub city: String,
  pub state: String,
  pub phone: String,
  pub birthday: DateTime<Utc>,
  pub favourite_dish: String
}

