 
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::app::PaymentDto;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Payment {
    pub id: String,
    pub order_id: String,
    pub amount: f32,
    pub payment_date: DateTime<Utc>,
    pub created_by: Option<String>,
    pub created_on: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    pub modfied_on: Option<DateTime<Utc>>,
}

impl azure_data_cosmos::CosmosEntity for Payment {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}

impl From<PaymentDto> for Payment {
    fn from(dto: PaymentDto) -> Self {
        Self {
            id: dto.id,
            order_id: dto.order_id,
            amount: dto.amount,
            payment_date: dto.payment_date,
            created_by: None,
            created_on: None,
            modified_by: None,
            modfied_on: None,
        }
    }
}
