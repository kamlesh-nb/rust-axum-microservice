use crate::{SharedCosmosRepository, SharedTelemetry, app::CustomerDto, domain::Customer};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 use appinsights::telemetry::SeverityLevel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateCustomerCommand {
  pub id: String,
  pub customer: CustomerDto,
}

impl Request<CustomerDto> for UpdateCustomerCommand {}

pub struct UpdateCustomerCommandHandler(pub SharedCosmosRepository<Customer>, pub DefaultAsyncMediator, pub SharedTelemetry);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateCustomerCommand, CustomerDto> for UpdateCustomerCommandHandler {
    async fn handle(&mut self, command: UpdateCustomerCommand) -> CustomerDto {
      let lock = self.0.lock().await;

      let name = env!("CARGO_PKG_NAME").to_string();
        let aai = self.2.lock().await;
        aai.track_trace(format!("[REQUEST] {},  {:?}", name, command.clone()), SeverityLevel::Information);

      let customer = lock.update(command.customer.into(), command.id.clone()).await.expect("Failed to update customer");
      
      aai.track_trace(format!("[RESPONSE] {}, UpdateCustomerCommand {:?}", name, customer.clone()), SeverityLevel::Information);
      customer.into()
    }
}