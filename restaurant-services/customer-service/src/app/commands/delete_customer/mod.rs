use crate::{SharedCosmosService, app::CustomerDto, domain::Customer};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomerCommand {
  pub id: String,
}

impl Request<CustomerDto> for DeleteCustomerCommand {}

pub struct DeleteCustomerCommandHandler(pub SharedCosmosService<Customer>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteCustomerCommand, CustomerDto> for DeleteCustomerCommandHandler {
    async fn handle(&mut self, command: DeleteCustomerCommand) -> CustomerDto {
      let lock = self.0.lock().await;
      let customer = lock.delete(command.id.clone()).await.expect("Failed to delete customer");
      customer
    }
}