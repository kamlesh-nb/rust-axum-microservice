use crate::domain::MenuCategory;
use crate::{app::MenuCategoryDto};
use crate::infra::SharedCosmosRepository;
use common::data::Repository;

use mediator::{AsyncRequestHandler, Request};

pub struct GetMenuCategoryByIdRequest{
  pub id: String
}

impl Request<Vec<MenuCategoryDto>> for GetMenuCategoryByIdRequest {}

pub struct GetMenuCategoryByIdRequestHandler(pub SharedCosmosRepository<MenuCategory>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetMenuCategoryByIdRequest, Vec<MenuCategoryDto>> for GetMenuCategoryByIdRequestHandler {

    async fn handle(&mut self, req: GetMenuCategoryByIdRequest) -> Vec<MenuCategoryDto> {
        let lock = self.0.lock().await;
        let menu_categories = lock.find_by_id(req.id).await.expect("no customer found");
        let mut menu_category_dtos: Vec<MenuCategoryDto> = Vec::new();
        for menu_category in menu_categories {
          menu_category_dtos.push(menu_category.into())
        }
        menu_category_dtos
    }
}