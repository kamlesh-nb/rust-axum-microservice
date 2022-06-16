use crate::{SharedCosmosRepository, domain::Payment};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeletePaymentCommand {
  pub id: String,
}

impl Request<String> for DeletePaymentCommand {}

pub struct DeletePaymentCommandHandler(pub SharedCosmosRepository<Payment>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeletePaymentCommand, String> for DeletePaymentCommandHandler {
    async fn handle(&mut self, command: DeletePaymentCommand) -> String {
      let lock = self.0.lock().await;
      let _ = lock.delete(command.id.clone()).await.expect("Failed to delete customer");
      command.id
    }
}