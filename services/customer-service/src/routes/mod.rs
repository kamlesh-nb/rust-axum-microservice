
use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};


use crate::{app::{
    CreateCustomerCommand, CustomerDto, CustomerDtoError, DeleteCustomerCommand,
    GetAllCustomersRequest, GetCustomerByIdRequest, UpdateCustomerCommand,
}};

use crate::infra::SharedMediator;

use mediator::AsyncMediator;

pub fn router() -> Router {
    Router::new()
        .route("/api/read/customers", get(get_all_customers))
        .route("/api/read/customer/:id", get(get_customer_by_id))
        .route("/api/create/customer", post(create_customer))
        .route("/api/update/customer/:id", put(update_customer))
        .route("/api/delete/customer/:id", delete(delete_customer))
}

#[utoipa::path(
get,
path = "/api/read/customers",
responses(
  (status = 200, description = "All customer items read successfully", body = [CustomerDto]),
  (status = 404, description = "Customers Not Found", body = CustomerDtoError),
  )
)]
pub async fn get_all_customers(
    Extension(mediator): Extension<SharedMediator>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(GetAllCustomersRequest {}).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(CustomerDtoError::NotFound(format!(
                "Customers not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}

#[utoipa::path(
get,
path = "/api/read/customer/{id}",
responses(
  (status = 200, description = "Customer items read successfully", body = CustomerDto),
  (status = 404, description = "Customer Not Found", body = CustomerDtoError),
  ),
  params(
    ("id" = String, path, description = "Customer ID")
),
)]
pub async fn get_customer_by_id(
    Extension(mediator): Extension<SharedMediator>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
   
    
    let result = mediator
        .send(GetCustomerByIdRequest {
            id: id.clone(),
        })
        .await;

    let out = match result {
        Ok(out) => match out.len() {
            1 => (StatusCode::OK, Json(out[0].clone())).into_response(),
            0 => (
                StatusCode::NOT_FOUND,
                Json(CustomerDtoError::NotFound(format!(
                    "Customer not found, ID: {}",
                    id
                ))),
            )
                .into_response(),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(CustomerDtoError::InternalServerError(
                    "Server Error".to_string(),
                )),
            )
                .into_response(),
        },
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomerDtoError::NotFound(format!("Server Error: {}", e))),
        )
            .into_response(),
    };
    out
}

#[utoipa::path(
post,
path = "/api/create/customer",
request_body = CustomerDto,
responses(
    (status = 201, description = "Customer item created successfully", body = CustomerDto),
    (status = 409, description = "Customer already exists", body = CustomerDtoError),
    (status = 500, description = "Internal Server Error", body = CustomerDtoError),
    )
)]
#[axum_macros::debug_handler]
pub async fn create_customer(
    Extension(mediator): Extension<SharedMediator>,
    Json(body): Json<CustomerDto>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;

    
    let result = mediator
        .send(CreateCustomerCommand { customer: body })
        .await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(CustomerDtoError::InternalServerError(format!(
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
path = "/api/update/customer/{id}",
request_body = CustomerDto,
responses(
    (status = 201, description = "Customer item updated successfully", body = CustomerDto),
    (status = 404, description = "Customer Not Found", body = CustomerDtoError),
    (status = 500, description = "Internal Server Error", body = CustomerDtoError),
    ),  params(
      ("id" = String, path, description = "Customer ID")
  ),
)]
#[axum_macros::debug_handler]
pub async fn update_customer(
    Extension(mediator): Extension<SharedMediator>,
    Path(id): Path<String>,
    Json(body): Json<CustomerDto>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator
        .send(UpdateCustomerCommand { customer: body, id })
        .await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(CustomerDtoError::NotFound(format!(
                "Customers not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}

#[utoipa::path(
delete,
path = "/api/delete/customer/{id}",
responses(
    (status = 201, description = "Customer item deleted successfully", body = String),
    (status = 404, description = "Customer Not Found", body = CustomerDtoError),
    (status = 500, description = "Internal Server Error", body = CustomerDtoError),
    ),
    params(
      ("id" = String, path, description = "Customer ID")
  ),
)]
#[axum_macros::debug_handler]
pub async fn delete_customer(
    Extension(mediator): Extension<SharedMediator>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(DeleteCustomerCommand { id }).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(CustomerDtoError::NotFound(format!(
                "Customers not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}
