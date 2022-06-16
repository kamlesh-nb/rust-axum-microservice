use crate::{SharedCosmosRepository, domain::IngredientCategory};
 
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteIngredientCategoryCommand {
  pub id: String,
}

impl Request<String> for DeleteIngredientCategoryCommand {}

pub struct DeleteIngredientCategoryCommandHandler(pub SharedCosmosRepository<IngredientCategory>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteIngredientCategoryCommand, String> for DeleteIngredientCategoryCommandHandler {
    async fn handle(&mut self, command: DeleteIngredientCategoryCommand) -> String {
      let lock = self.0.lock().await;
      let _ = lock.delete(command.id.clone()).await.expect("Failed to delete customer");
      command.id
    }
}