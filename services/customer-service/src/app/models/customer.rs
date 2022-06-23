use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};
use validator::Validate;

use crate::domain::Customer;

#[derive(Serialize, Deserialize, Validate, Component, Clone, Debug)]
pub struct CustomerDto {
  pub id: String,

  #[validate(length(min = 1, max = 15))]
  pub first_name: String,

  #[validate(length(min = 1, max = 15))]
  pub last_name: String,

  #[validate(email)]
  pub email: String,
  pub address: String,
  pub city: String,
  pub state: String,
  pub phone: String,
  pub birthday: DateTime<Utc>,
  pub favourite_dishes: Vec<String>
}

impl From<Customer> for CustomerDto {
  fn from(entity: Customer) -> Self {
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

