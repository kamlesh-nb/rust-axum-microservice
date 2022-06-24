use serde::{Deserialize, Serialize};
use utoipa::Component;
use chrono::{DateTime, Utc};

use crate::domain::Payment;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct PaymentDto {
  pub id: String,
  pub order_id: String,
  pub amount: f32,
  pub payment_date: DateTime<Utc>,
  pub processed_by: String,
}

impl From<Payment> for PaymentDto {
  fn from(entity: Payment) -> Self {
      Self {
          id: entity.id,
          order_id: entity.order_id,
          amount: entity.amount,
          payment_date: entity.payment_date,
          processed_by: Some(entity.created_by).unwrap().unwrap(),
      }
  }
}

