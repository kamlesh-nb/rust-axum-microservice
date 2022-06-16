use axum::{
  extract::Path,
  http::StatusCode,
  response::IntoResponse,
  routing::{delete, get, post, put},
  Extension, Json, Router,
};

use crate::app::{
  CreateRatingCommand, RatingDto, RatingDtoError, DeleteRatingCommand,
  GetAllRatingsRequest, GetRatingByIdRequest, UpdateRatingCommand,
};

use crate::infra::SharedMediator;

use mediator::AsyncMediator;

pub fn router() -> Router {
  Router::new()
      .route("/api/read/ratings", get(get_all_ratings))
      .route("/api/read/rating/:id", get(get_rating_by_id))
      .route("/api/create/rating", post(create_rating))
      .route("/api/update/rating/:id", put(update_rating))
      .route("/api/delete/rating/:id", delete(delete_rating))
}

#[utoipa::path(
get,
path = "/api/read/ratings",
responses(
(status = 200, description = "All rating items read successfully", body = [RatingDto]),
(status = 404, description = "Ratings Not Found", body = RatingDtoError),
)
)]
pub async fn get_all_ratings(
  Extension(mediator): Extension<SharedMediator>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator.send(GetAllRatingsRequest {}).await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(RatingDtoError::NotFound(format!(
              "Ratings not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
get,
path = "/api/read/rating/{id}",
responses(
(status = 200, description = "Rating items read successfully", body = RatingDto),
(status = 404, description = "Rating Not Found", body = RatingDtoError),
),
params(
  ("id" = String, path, description = "Rating ID")
),
)]
pub async fn get_rating_by_id(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(GetRatingByIdRequest {
          id: id.clone(),
      })
      .await;

  let out = match result {
      Ok(out) => match out.len() {
          1 => (StatusCode::OK, Json(out[0].clone())).into_response(),
          0 => (
              StatusCode::NOT_FOUND,
              Json(RatingDtoError::NotFound(format!(
                  "Rating not found, ID: {}",
                  id
              ))),
          )
              .into_response(),
          _ => (
              StatusCode::INTERNAL_SERVER_ERROR,
              Json(RatingDtoError::InternalServerError(
                  "Server Error".to_string(),
              )),
          )
              .into_response(),
      },
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(RatingDtoError::NotFound(format!("Server Error: {}", e))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
post,
path = "/api/create/rating",
request_body = RatingDto,
responses(
  (status = 201, description = "Rating item created successfully", body = RatingDto),
  (status = 409, description = "Rating already exists", body = RatingDtoError),
  (status = 500, description = "Internal Server Error", body = RatingDtoError),
  )
)]
#[axum_macros::debug_handler]
pub async fn create_rating(
  Extension(mediator): Extension<SharedMediator>,
  Json(body): Json<RatingDto>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(CreateRatingCommand {rating: body })
      .await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(RatingDtoError::InternalServerError(format!(
              "Server Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
put,
path = "/api/update/rating/{id}",
request_body = RatingDto,
responses(
  (status = 201, description = "Rating item updated successfully", body = RatingDto),
  (status = 404, description = "Rating Not Found", body = RatingDtoError),
  (status = 500, description = "Internal Server Error", body = RatingDtoError),
  ),  params(
    ("id" = String, path, description = "Rating ID")
),
)]
#[axum_macros::debug_handler]
pub async fn update_rating(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
  Json(body): Json<RatingDto>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(UpdateRatingCommand { rating: body, id })
      .await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(RatingDtoError::NotFound(format!(
              "Ratings not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
delete,
path = "/api/delete/rating/{id}",
responses(
  (status = 201, description = "Rating item deleted successfully", body = String),
  (status = 404, description = "Rating Not Found", body = RatingDtoError),
  (status = 500, description = "Internal Server Error", body = RatingDtoError),
  ),
  params(
    ("id" = String, path, description = "Rating ID")
),
)]
#[axum_macros::debug_handler]
pub async fn delete_rating(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator.send(DeleteRatingCommand { id }).await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(RatingDtoError::NotFound(format!(
              "Ratings not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}
