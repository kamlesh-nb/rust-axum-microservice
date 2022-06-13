use crate::{SharedCosmosService, app::Customer};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerCommand {
  pub customer: Customer,
}

impl Request<Customer> for CreateCustomerCommand {}

pub struct CreateCustomerCommandHandler(pub SharedCosmosService, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateCustomerCommand, Customer> for CreateCustomerCommandHandler {
    async fn handle(&mut self, command: CreateCustomerCommand) -> Customer {
      let lock = self.0.lock().await;
      let customer = lock.create(command.customer).await.expect("Failed to create customer");
      customer  
    }
}