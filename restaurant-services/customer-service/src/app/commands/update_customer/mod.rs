use crate::{SharedCosmosService, app::CustomerDto, domain::Customer};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerCommand {
  pub customer: CustomerDto,
}

impl Request<CustomerDto> for UpdateCustomerCommand {}

pub struct UpdateCustomerCommandHandler(pub SharedCosmosService<Customer>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateCustomerCommand, CustomerDto> for UpdateCustomerCommandHandler {
    async fn handle(&mut self, command: UpdateCustomerCommand) -> CustomerDto {
      let lock = self.0.lock().await;
      let customer = lock.update(command.customer, command.customer.id.clone()).await.expect("Failed to update customer");
      customer
    }
}