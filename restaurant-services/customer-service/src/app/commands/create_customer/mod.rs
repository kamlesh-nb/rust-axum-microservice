use crate::{SharedCosmosService, app::CustomerDto, domain::Customer};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerCommand {
  pub customer: CustomerDto,
}

impl Request<CustomerDto> for CreateCustomerCommand {}

pub struct CreateCustomerCommandHandler(pub SharedCosmosService<Customer>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateCustomerCommand, CustomerDto> for CreateCustomerCommandHandler {
    async fn handle(&mut self, command: CreateCustomerCommand) -> CustomerDto {
      let lock = self.0.lock().await;
      let customer = lock.create(command.customer).await.expect("Failed to create customer");
      customer  
    }
}