use crate::{SharedCosmosService, app::Customer};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerCommand {
  pub customer: Customer,
}

impl Request<Customer> for UpdateCustomerCommand {}

pub struct UpdateCustomerCommandHandler(pub SharedCosmosService, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateCustomerCommand, Customer> for UpdateCustomerCommandHandler {
    async fn handle(&mut self, command: UpdateCustomerCommand) -> Customer {
      let lock = self.0.lock().await;
      let customer = lock.update(command.customer).await.expect("Failed to update customer");
      customer
    }
}