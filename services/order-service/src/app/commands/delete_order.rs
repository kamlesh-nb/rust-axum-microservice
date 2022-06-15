use crate::{SharedCosmosRepository, domain::Order};
 
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOrderCommand {
  pub id: String,
}

impl Request<String> for DeleteOrderCommand {}

pub struct DeleteOrderCommandHandler(pub SharedCosmosRepository<Order>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteOrderCommand, String> for DeleteOrderCommandHandler {
    async fn handle(&mut self, command: DeleteOrderCommand) -> String {
      let lock = self.0.lock().await;
      let _ = lock.delete(command.id.clone()).await.expect("Failed to delete customer");
      command.id
    }
}