use crate::{SharedCosmosRepository, app::CustomerDto, domain::Customer};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerCommand {
  pub customer: CustomerDto,
}

impl Request<CustomerDto> for CreateCustomerCommand {}

pub struct CreateCustomerCommandHandler(pub SharedCosmosRepository<Customer>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateCustomerCommand, CustomerDto> for CreateCustomerCommandHandler {
    async fn handle(&mut self, command: CreateCustomerCommand) -> CustomerDto {
      let lock = self.0.lock().await;
      let customer = lock.create(command.customer.into()).await.expect("Failed to create customer");
      customer.into()  
    }
}