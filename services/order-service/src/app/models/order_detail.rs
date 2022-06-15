use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct OrderDetailDto {
    pub menu_id: i32,
    pub quantity: i8,
    pub price: f32,
    pub total: f32
}