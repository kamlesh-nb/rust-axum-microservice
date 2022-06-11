use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct OrderDetail {
    pub menu_id: i32,
    pub quantity: i8,
    pub price: f32,
    pub total: f32
}