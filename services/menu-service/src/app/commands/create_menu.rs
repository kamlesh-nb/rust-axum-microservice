use crate::{app::MenuCategoryDto, domain::MenuCategory, infra::SharedCosmosRepository};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
use common::data::Repository;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateMenuCategoryCommand {
  pub menu_category: MenuCategoryDto,
}

impl Request<MenuCategoryDto> for CreateMenuCategoryCommand {}

pub struct CreateMenuCategoryCommandHandler(pub SharedCosmosRepository<MenuCategory>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateMenuCategoryCommand, MenuCategoryDto> for CreateMenuCategoryCommandHandler {
    async fn handle(&mut self, command: CreateMenuCategoryCommand) -> MenuCategoryDto {
      let lock = self.0.lock().await;
      let menu_category = lock.create(command.menu_category.into()).await.expect("Failed to create MenuCategory");
      menu_category.into()  
    }
}