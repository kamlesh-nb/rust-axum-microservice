use axum::{
  extract::Path,
  http::StatusCode,
  response::IntoResponse,
  routing::{delete, get, post, put},
  Extension, Json, Router,
};


pub fn router() -> Router {
  Router::new()
}