use axum::{
  extract::Path,
  http::StatusCode,
  response::IntoResponse,
  routing::{delete, get, post, put},
  Extension, Json, Router,
};

 

use crate::{infra::SharedMediator, app::{MenuCategoryDtoError, GetAllMenuCategoriesRequest, GetMenuCategoryByIdRequest, MenuCategoryDto, CreateMenuCategoryCommand, UpdateMenuCategoryCommand, DeleteMenuCategoryCommand}};

use mediator::AsyncMediator;

pub fn router() -> Router {
  Router::new()
      .route("/api/read/menus", get(get_all_menus))
      .route("/api/read/menu/:id", get(get_menu_by_id))
      .route("/api/create/menu", post(create_menu))
      .route("/api/update/menu/:id", put(update_menu))
      .route("/api/delete/menu/:id", delete(delete_menu))
}

#[utoipa::path(
get,
path = "/api/read/menus",
responses(
(status = 200, description = "All menu items read successfully", body = [MenuCategoryDto]),
(status = 404, description = "MenuCategorys Not Found", body = MenuCategoryDtoError),
)
)]
pub async fn get_all_menus(
  Extension(mediator): Extension<SharedMediator>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator.send(GetAllMenuCategoriesRequest {}).await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(MenuCategoryDtoError::NotFound(format!(
              "MenuCategorys not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
get,
path = "/api/read/menu/{id}",
responses(
(status = 200, description = "MenuCategory items read successfully", body = MenuCategoryDto),
(status = 404, description = "MenuCategory Not Found", body = MenuCategoryDtoError),
),
params(
  ("id" = String, path, description = "MenuCategory ID")
),
)]
pub async fn get_menu_by_id(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(GetMenuCategoryByIdRequest {
          id: id.clone(),
      })
      .await;

  let out = match result {
      Ok(out) => match out.len() {
          1 => (StatusCode::OK, Json(out[0].clone())).into_response(),
          0 => (
              StatusCode::NOT_FOUND,
              Json(MenuCategoryDtoError::NotFound(format!(
                  "MenuCategory not found, ID: {}",
                  id
              ))),
          )
              .into_response(),
          _ => (
              StatusCode::INTERNAL_SERVER_ERROR,
              Json(MenuCategoryDtoError::InternalServerError(
                  "Server Error".to_string(),
              )),
          )
              .into_response(),
      },
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(MenuCategoryDtoError::NotFound(format!("Server Error: {}", e))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
post,
path = "/api/create/menu",
request_body = MenuCategoryDto,
responses(
  (status = 201, description = "MenuCategory item created successfully", body = MenuCategoryDto),
  (status = 409, description = "MenuCategory already exists", body = MenuCategoryDtoError),
  (status = 500, description = "Internal Server Error", body = MenuCategoryDtoError),
  )
)]
#[axum_macros::debug_handler]
pub async fn create_menu(
  Extension(mediator): Extension<SharedMediator>,
  Json(body): Json<MenuCategoryDto>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(CreateMenuCategoryCommand { menu_category: body })
      .await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::INTERNAL_SERVER_ERROR,
          Json(MenuCategoryDtoError::InternalServerError(format!(
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
path = "/api/update/menu/{id}",
request_body = MenuCategoryDto,
responses(
  (status = 201, description = "MenuCategory item updated successfully", body = MenuCategoryDto),
  (status = 404, description = "MenuCategory Not Found", body = MenuCategoryDtoError),
  (status = 500, description = "Internal Server Error", body = MenuCategoryDtoError),
  ),  params(
    ("id" = String, path, description = "MenuCategory ID")
),
)]
#[axum_macros::debug_handler]
pub async fn update_menu(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
  Json(body): Json<MenuCategoryDto>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator
      .send(UpdateMenuCategoryCommand { menu_category: body, id })
      .await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(MenuCategoryDtoError::NotFound(format!(
              "MenuCategorys not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}

#[utoipa::path(
delete,
path = "/api/delete/menu/{id}",
responses(
  (status = 201, description = "MenuCategory item deleted successfully", body = String),
  (status = 404, description = "MenuCategory Not Found", body = MenuCategoryDtoError),
  (status = 500, description = "Internal Server Error", body = MenuCategoryDtoError),
  ),
  params(
    ("id" = String, path, description = "MenuCategory ID")
),
)]
#[axum_macros::debug_handler]
pub async fn delete_menu(
  Extension(mediator): Extension<SharedMediator>,
  Path(id): Path<String>,
) -> impl IntoResponse {
  let mut mediator = mediator.lock().await;
  let result = mediator.send(DeleteMenuCategoryCommand { id }).await;

  let out = match result {
      Ok(out) => (StatusCode::OK, Json(out)).into_response(),
      Err(e) => (
          StatusCode::NOT_FOUND,
          Json(MenuCategoryDtoError::NotFound(format!(
              "MenuCategorys not found, Error: {}",
              e
          ))),
      )
          .into_response(),
  };
  out
}
