use crate::{app::MenuCategoryDto};
use crate::domain::MenuCategory;
use crate::{SharedCosmosRepository};
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetAllMenuCategoriesRequest{
}

impl Request<Vec<MenuCategoryDto>> for GetAllMenuCategoriesRequest {}

pub struct GetAllMenuCategoriesRequestHandler(pub SharedCosmosRepository<MenuCategory>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetAllMenuCategoriesRequest, Vec<MenuCategoryDto>> for GetAllMenuCategoriesRequestHandler {

    async fn handle(&mut self, _req: GetAllMenuCategoriesRequest) -> Vec<MenuCategoryDto> {
        let lock = self.0.lock().await;
        let menu_categories = lock.find_all().await.expect("no customer found");
        let mut menu_category_dtos: Vec<MenuCategoryDto> = Vec::new();
        for menu_category in menu_categories {
          menu_category_dtos.push(menu_category.into())
        }
        menu_category_dtos
    }
}