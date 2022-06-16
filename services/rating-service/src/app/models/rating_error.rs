use serde::{Serialize, Deserialize};
use utoipa::Component;



#[derive(Serialize, Deserialize, Component, Clone, Debug)]
  pub enum RatingDtoError {
      #[component(example = "Rating already exists")]
      Conflict(String),
      #[component(example = "id = 1")]
      NotFound(String),
      #[component(example = "Internal Server Error")]
      InternalServerError(String),
  }