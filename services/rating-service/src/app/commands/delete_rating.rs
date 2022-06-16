use crate::{SharedCosmosRepository, domain::Rating};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteRatingCommand {
  pub id: String,
}

impl Request<String> for DeleteRatingCommand {}

pub struct DeleteRatingCommandHandler(pub SharedCosmosRepository<Rating>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteRatingCommand, String> for DeleteRatingCommandHandler {
    async fn handle(&mut self, command: DeleteRatingCommand) -> String {
      let lock = self.0.lock().await;
      let _ = lock.delete(command.id.clone()).await.expect("Failed to delete customer");
      command.id
    }
}