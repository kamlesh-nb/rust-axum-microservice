use axum::{
  extract::Path,
  http::StatusCode,
  response::IntoResponse,
  routing::{delete, get, post, put},
  Extension, Json, Router,
};

use crate::app::{
  CreatePaymentCommand, PaymentDto, PaymentDtoError, DeletePaymentCommand,
  GetAllPaymentsRequest, GetPaymentByIdRequest, UpdatePaymentCommand,
};

use crate::infra::SharedMediator;

use mediator::AsyncMediator;

pub fn router() -> Router {
  Router::new()
      .route("/api/read/payments", get(get_all_payments))
      .route("/api/read/payment/:id", get(get_payment_by_id))
      .route("/api/create/payment", post(create_payment))
      .route("/api/update/payment/:id", put(update_payment))
      .route("/api/delete/payment/:id", delete(delete_payment))
}

#[utoipa::path(
get,
path = "/api/read/payments",
responses(
(status = 200, description = "All payment items read successfully", body = [PaymentDto]),
(status = 404, description = "Payments Not Found", body = PaymentDtoError),
)
)]
pub async fn get_all_payments(
  Extension(mediator): Extension<SharedMediator>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator.send(GetAllPaymentsRequest {}).await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(PaymentDtoError::NotFound(format!(
              "Payments not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
get,
path = "/api/read/payment/{id}",
responses(
(status = 200, description = "Payment items read successfully", body = PaymentDto),
(status = 404, description = "Payment Not Found", body = PaymentDtoError),
),
params(
  ("id" = String, path, description = "Payment ID")
),
)]
pub async fn get_payment_by_id(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(GetPaymentByIdRequest {
          id: id.clone(),
      })
      .await;

  let out = match result {
      Ok(out) => match out.len() {
          1 => (StatusCode::OK, Json(out[0].clone())).into_response(),
          0 => (
              StatusCode::NOT_FOUND,
              Json(PaymentDtoError::NotFound(format!(
                  "Payment not found, ID: {}",
                  id
              ))),
          )
              .into_response(),
          _ => (
              StatusCode::INTERNAL_SERVER_ERROR,
              Json(PaymentDtoError::InternalServerError(
                  "Server Error".to_string(),
              )),
          )
              .into_response(),
      },
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(PaymentDtoError::NotFound(format!("Server Error: {}", e))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
post,
path = "/api/create/payment",
request_body = PaymentDto,
responses(
  (status = 201, description = "Payment item created successfully", body = PaymentDto),
  (status = 409, description = "Payment already exists", body = PaymentDtoError),
  (status = 500, description = "Internal Server Error", body = PaymentDtoError),
  )
)]
#[axum_macros::debug_handler]
pub async fn create_payment(
  Extension(mediator): Extension<SharedMediator>,
  Json(body): Json<PaymentDto>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(CreatePaymentCommand {payment:body })
      .await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(PaymentDtoError::InternalServerError(format!(
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
path = "/api/update/payment/{id}",
request_body = PaymentDto,
responses(
  (status = 201, description = "Payment item updated successfully", body = PaymentDto),
  (status = 404, description = "Payment Not Found", body = PaymentDtoError),
  (status = 500, description = "Internal Server Error", body = PaymentDtoError),
  ),  params(
    ("id" = String, path, description = "Payment ID")
),
)]
#[axum_macros::debug_handler]
pub async fn update_payment(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
  Json(body): Json<PaymentDto>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(UpdatePaymentCommand { payment: body, id })
      .await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(PaymentDtoError::NotFound(format!(
              "Payments not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
delete,
path = "/api/delete/payment/{id}",
responses(
  (status = 201, description = "Payment item deleted successfully", body = String),
  (status = 404, description = "Payment Not Found", body = PaymentDtoError),
  (status = 500, description = "Internal Server Error", body = PaymentDtoError),
  ),
  params(
    ("id" = String, path, description = "Payment ID")
),
)]
#[axum_macros::debug_handler]
pub async fn delete_payment(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator.send(DeletePaymentCommand { id }).await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(PaymentDtoError::NotFound(format!(
              "Payments not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}
