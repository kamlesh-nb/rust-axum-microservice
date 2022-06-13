use crate::{SharedCosmosService, app::Customer};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomerCommand {
  pub customer: Customer,
}

impl Request<Customer> for DeleteCustomerCommand {}

pub struct DeleteCustomerCommandHandler(pub SharedCosmosService, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteCustomerCommand, Customer> for DeleteCustomerCommandHandler {
    async fn handle(&mut self, command: DeleteCustomerCommand) -> Customer {
      let lock = self.0.lock().await;
      let customer = lock.delete(command.customer).await.expect("Failed to delete customer");
      customer
    }
}