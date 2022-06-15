use azure_data_cosmos::CosmosEntity;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::Component;

use crate::{domain, app};

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct Order {
    pub id: String,
    pub order_date: DateTime<Utc>,
    pub customer_id: i32,
    pub total_amount: f32,
    pub order_status: i8,
    pub order_details: Vec<super::OrderDetail>,
    pub created_by: Option<String>,
    pub created_on: Option<DateTime<Utc>>,
    pub modified_by: Option<String>,
    pub modfied_on: Option<DateTime<Utc>>,
}

impl CosmosEntity for Order {
    type Entity = String;

    fn partition_key(&self) -> Self::Entity {
        self.id.clone()
    }
}


impl From<app::OrderDto> for Order {
    fn from(order: app::OrderDto) -> Self {
        let mut orderdetails: Vec<domain::OrderDetail> = Vec::new();
        for od in order.order_details {
          orderdetails.push(domain::OrderDetail{
            menu_id: od.menu_id,
            price: od.price,
            quantity: od.quantity,
            total: od.total
          });
        }

        Self { id: order.id, 
          order_date: order.order_date, 
          customer_id: order.customer_id, 
          total_amount: order.total_amount, 
          order_status: order.order_status, 
          order_details: orderdetails, 
          created_by: None, 
          created_on: None, 
          modified_by: None, 
          modfied_on: None 
        }
    }
}