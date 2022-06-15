use crate::{SharedCosmosRepository, app::CustomerDto, domain::Customer};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerCommand {
  pub id: String,
  pub customer: CustomerDto,
}

impl Request<CustomerDto> for UpdateCustomerCommand {}

pub struct UpdateCustomerCommandHandler(pub SharedCosmosRepository<Customer>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateCustomerCommand, CustomerDto> for UpdateCustomerCommandHandler {
    async fn handle(&mut self, command: UpdateCustomerCommand) -> CustomerDto {
      let lock = self.0.lock().await;
      let customer = lock.update(command.customer.into(), command.id.clone()).await.expect("Failed to update customer");
      customer.into()
    }
}