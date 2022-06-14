use serde::{Serialize, Deserialize};
use utoipa::Component;



#[derive(Serialize, Deserialize, Component, Clone, Debug)]
  pub enum OrderError {
      #[component(example = "Order already exists")]
      Conflict(String),
      #[component(example = "id = 1")]
      NotFound(String),
      #[component(example = "Internal Server Error")]
      InternalServerError(String),
  }