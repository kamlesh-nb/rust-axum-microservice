use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use utoipa::Component;

use crate::{domain, app};
 

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct OrderDto {
    pub id: String,
    pub order_date: DateTime<Utc>,
    pub customer_id: i32,
    pub total_amount: f32,
    pub order_status: i8,
    pub order_details: Vec<super::OrderDetailDto>,
    pub processed_by: String,
}


impl From<domain::Order> for OrderDto {
  fn from(order: domain::Order) -> Self {
      let mut orderdetails: Vec<app::OrderDetailDto> = Vec::new();
      for od in order.order_details {
        orderdetails.push(app::OrderDetailDto{
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
        processed_by: Some(order.created_by).unwrap().unwrap(), 
      }
  }
}