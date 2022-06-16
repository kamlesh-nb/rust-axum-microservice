use axum::{
    extract::Path,
    http::StatusCode,
    response::IntoResponse,
    routing::{delete, get, post, put},
    Extension, Json, Router,
};

use crate::{app::{
    CreateIngredientCategoryCommand, DeleteIngredientCategoryCommand, GetAllIngredientCategorysRequest, GetIngredientCategoryByIdRequest, IngredientCategoryDto,
    IngredientError,
}, infra::SharedMediator};
use crate::app::UpdateIngredientCategoryCommand;
use mediator::AsyncMediator;

pub fn router() -> Router {
    Router::new()
        .route("/api/read/ingredient_categorys", get(get_all_ingredient_categories))
        .route("/api/read/ingredient_category/:id", get(get_ingredient_category_by_id))
        .route("/api/create/ingredient_category", post(create_ingredient_category))
        .route("/api/update/ingredient_category", put(update_ingredient_category))
        .route("/api/delete/ingredient_category", delete(delete_ingredient_category))
}

#[utoipa::path(
  get,
  path = "/api/read/ingredient_categories",
  responses(
    (status = 200, description = "All IngredientCategory items read successfully", body = [IngredientCategoryDto]),
    (status = 404, description = "IngredientCategorys Not Found", body = IngredientError),
    )
)]
pub async fn get_all_ingredient_categories(Extension(mediator): Extension<SharedMediator>) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(GetAllIngredientCategorysRequest {}).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(IngredientError::NotFound(format!(
                "IngredientCategorys not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}

#[utoipa::path(
  get,
  path = "/api/read/ingredient_category/{id}",
  responses(
    (status = 200, description = "IngredientCategory items read successfully", body = IngredientCategoryDto),
    (status = 404, description = "IngredientCategory Not Found", body = IngredientError),
    ),
    params(
      ("id" = String, path, description = "IngredientCategory ID")
  ),
)]
pub async fn get_ingredient_category_by_id(
    Extension(mediator): Extension<SharedMediator>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(GetIngredientCategoryByIdRequest { id: id.clone() }).await;

    let out = match result {
        Ok(out) => match out.len() {
          1 => (StatusCode::OK, Json(out[0].clone())).into_response(),
          0 => (StatusCode::NOT_FOUND, Json(IngredientError::NotFound(format!("IngredientCategory not found, ID: {}",id))),).into_response(),
          _ => (StatusCode::INTERNAL_SERVER_ERROR, Json(IngredientError::InternalServerError("Server Error".to_string()))).into_response(),
        },
        Err(e) => (StatusCode::INTERNAL_SERVER_ERROR, Json(IngredientError::NotFound(format!("Server Error: {}",e))),).into_response(),
    };
    out
}

#[utoipa::path(
  post,
  path = "/api/create/ingredient_category",
  request_body = IngredientCategoryDto,
  responses(
      (status = 201, description = "IngredientCategory item created successfully", body = IngredientCategoryDto),
      (status = 409, description = "IngredientCategory already exists", body = IngredientError),
      (status = 500, description = "Internal Server Error", body = IngredientError),
      )
)]
#[axum_macros::debug_handler]
pub async fn create_ingredient_category(
    Extension(mediator): Extension<SharedMediator>,
    Json(body): Json<IngredientCategoryDto>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(CreateIngredientCategoryCommand { ingredient_category: body }).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(IngredientError::InternalServerError(format!(
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
  path = "/api/update/ingredient_category/{id}",
  request_body = IngredientCategoryDto,
  responses(
      (status = 201, description = "IngredientCategory item updated successfully", body = IngredientCategoryDto),
      (status = 404, description = "IngredientCategory Not Found", body = IngredientError),
      (status = 500, description = "Internal Server Error", body = IngredientError),
      ),
      params(
        ("id" = String, path, description = "IngredientCategory ID")
    ),
)]
#[axum_macros::debug_handler]
pub async fn update_ingredient_category(
    Extension(mediator): Extension<SharedMediator>,
    Path(id): Path<String>,
    Json(body): Json<IngredientCategoryDto>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(UpdateIngredientCategoryCommand {ingredient_category: body, id }).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(IngredientError::NotFound(format!(
                "IngredientCategorys not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}

#[utoipa::path(
  delete,
  path = "/api/delete/ingredient_category/{id}",
  request_body = IngredientCategoryDto,
  responses(
      (status = 201, description = "IngredientCategory item deleted successfully", body = IngredientCategoryDto),
      (status = 404, description = "IngredientCategory Not Found", body = IngredientError),
      (status = 500, description = "Internal Server Error", body = IngredientError),
      ),
      params(
        ("id" = String, path, description = "IngredientCategory ID")
    ),
)]
#[axum_macros::debug_handler]
pub async fn delete_ingredient_category(
    Extension(mediator): Extension<SharedMediator>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    let mut mediator = mediator.lock().await;
    let result = mediator.send(DeleteIngredientCategoryCommand { id }).await;

    let out = match result {
        Ok(out) => (StatusCode::OK, Json(out)).into_response(),
        Err(e) => (
            StatusCode::NOT_FOUND,
            Json(IngredientError::NotFound(format!(
                "IngredientCategorys not found, Error: {}",
                e
            ))),
        )
            .into_response(),
    };
    out
}
