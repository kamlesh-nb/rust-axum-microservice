use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};

use crate::app::{
    CreateOrderCommand, DeleteOrderCommand, GetAllOrdersRequest, GetOrderByIdRequest, Order,
    OrderError, SharedMediator, UpdateOrderCommand,
};
use mediator::AsyncMediator;

pub fn router() -> Router {
    Router::new()
        .route("/api/read/orders", get(get_all_orders))
        .route("/api/read/order/:order_id", get(get_order_by_id))
        .route("/api/create/order", post(create_order))
        .route("/api/update/order", put(update_order))
        .route("/api/delete/order", delete(delete_order))
}

#[utoipa::path(
  get,
  path = "/api/read/orders",
  responses(
    (status = 200, description = "All Order items read successfully", body = [Order]),
    (status = 404, description = "Orders Not Found", body = OrderError),
    )
)]
pub async fn get_all_orders(Extension(mediator): Extension<SharedMediator>) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(GetAllOrdersRequest {}).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(OrderError::NotFound(format!(
                "Orders not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}

#[utoipa::path(
  get,
  path = "/api/read/order/{order_id}",
  responses(
    (status = 200, description = "Order items read successfully", body = Order),
    (status = 404, description = "Order Not Found", body = OrderError),
    ),
    params(
      ("order_id" = String, path, description = "Order ID")
  ),
)]
pub async fn get_order_by_id(
    Extension(mediator): Extension<SharedMediator>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(GetOrderByIdRequest { order_id: id.clone() }).await;

    let out = match result {
        Ok(out) => match out.len() {
          1 => (StatusCode::OK, Json(out[0].clone())).into_response(),
          0 => (StatusCode::NOT_FOUND, Json(OrderError::NotFound(format!("Order not found, ID: {}",id))),).into_response(),
          _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(OrderError::InternalServerError("Server Error".to_string()))).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(OrderError::NotFound(format!("Server Error: {}",e))),).into_response(),
    };
    out
}

#[utoipa::path(
  post,
  path = "/api/create/order",
  request_body = Order,
  responses(
      (status = 201, description = "Order item created successfully", body = Order),
      (status = 409, description = "Order already exists", body = OrderError),
      (status = 500, description = "Internal Server Error", body = OrderError),
      )
)]
#[axum_macros::debug_handler]
pub async fn create_order(
    Extension(mediator): Extension<SharedMediator>,
    Json(body): Json<Order>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(CreateOrderCommand { order: body }).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(OrderError::InternalServerError(format!(
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
  path = "/api/update/order",
  request_body = Order,
  responses(
      (status = 201, description = "Order item updated successfully", body = Order),
      (status = 404, description = "Order Not Found", body = OrderError),
      (status = 500, description = "Internal Server Error", body = OrderError),
      )
)]
#[axum_macros::debug_handler]
pub async fn update_order(
    Extension(mediator): Extension<SharedMediator>,
    Json(body): Json<Order>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(UpdateOrderCommand { order: body }).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(OrderError::NotFound(format!(
                "Orders not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}

#[utoipa::path(
  delete,
  path = "/api/delete/order",
  request_body = Order,
  responses(
      (status = 201, description = "Order item deleted successfully", body = Order),
      (status = 404, description = "Order Not Found", body = OrderError),
      (status = 500, description = "Internal Server Error", body = OrderError),
      )
)]
#[axum_macros::debug_handler]
pub async fn delete_order(
    Extension(mediator): Extension<SharedMediator>,
    Json(body): Json<Order>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(DeleteOrderCommand { order: body }).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(OrderError::NotFound(format!(
                "Orders not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}
