use azure_data_cosmos::CosmosEntity;
use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Customer {
  pub id: String,
  pub first_name: String,
  pub last_name: String,
  pub email: String,
  pub address: String,
  pub city: String,
  pub state: String,
  pub phone: String,
  pub birthday: DateTime<Utc>,
  pub favourite_dish: Vec<String>,
  pub created_by: Option<String>,
  pub created_on: Option<DateTime<Utc>>,
  pub modified_by: Option<String>,
  pub modfied_on: Option<DateTime<Utc>>,
}

impl CosmosEntity for Customer {
  type Entity = String;

  fn partition_key(&self) -> Self::Entity {
      self.id.clone()
  }
}