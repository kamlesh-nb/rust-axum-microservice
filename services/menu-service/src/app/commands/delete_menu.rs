use crate::{SharedCosmosRepository, domain::MenuCategory};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteMenuCategoryCommand {
  pub id: String,
}

impl Request<String> for DeleteMenuCategoryCommand {}

pub struct DeleteMenuCategoryCommandHandler(pub SharedCosmosRepository<MenuCategory>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteMenuCategoryCommand, String> for DeleteMenuCategoryCommandHandler {
    async fn handle(&mut self, command: DeleteMenuCategoryCommand) -> String {
      let lock = self.0.lock().await;
      let _ = lock.delete(command.id.clone()).await.expect("Failed to delete MenuCategory");
      command.id
    }
}