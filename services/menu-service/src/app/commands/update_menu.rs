use crate::{SharedCosmosRepository, app::MenuCategoryDto, domain::MenuCategory};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateMenuCategoryCommand {
  pub id: String,
  pub menu_category: MenuCategoryDto,
}

impl Request<MenuCategoryDto> for UpdateMenuCategoryCommand {}

pub struct UpdateMenuCategoryCommandHandler(pub SharedCosmosRepository<MenuCategory>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateMenuCategoryCommand, MenuCategoryDto> for UpdateMenuCategoryCommandHandler {
    async fn handle(&mut self, command: UpdateMenuCategoryCommand) -> MenuCategoryDto {
      let lock = self.0.lock().await;
      let menu_category = lock.update(command.menu_category.into(), command.id.clone()).await.expect("Failed to update MenuCategory");
      menu_category.into()
    }
}