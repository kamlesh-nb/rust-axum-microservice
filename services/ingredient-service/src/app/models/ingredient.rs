use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Serialize, Deserialize, Component, Clone, Debug)]
pub struct IngredientDto {
    pub id: i32,
    pub name: String,
    pub uom: String,
}
