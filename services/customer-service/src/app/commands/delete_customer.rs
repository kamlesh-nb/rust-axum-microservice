use crate::{SharedCosmosRepository, domain::Customer};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomerCommand {
  pub id: String,
}

impl Request<String> for DeleteCustomerCommand {}

pub struct DeleteCustomerCommandHandler(pub SharedCosmosRepository<Customer>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteCustomerCommand, String> for DeleteCustomerCommandHandler {
    async fn handle(&mut self, command: DeleteCustomerCommand) -> String {
      let lock = self.0.lock().await;
      let _ = lock.delete(command.id.clone()).await.expect("Failed to delete customer");
      command.id
    }
}