use crate::{SharedCosmosRepository, SharedTelemetry, app::CustomerDto, domain::Customer};
use appinsights::telemetry::SeverityLevel;
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateCustomerCommand {
  pub customer: CustomerDto,
}

impl Request<CustomerDto> for CreateCustomerCommand {}

pub struct CreateCustomerCommandHandler(pub SharedCosmosRepository<Customer>, pub DefaultAsyncMediator, pub SharedTelemetry);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateCustomerCommand, CustomerDto> for CreateCustomerCommandHandler {
    async fn handle(&mut self, command: CreateCustomerCommand) -> CustomerDto {
      let lock = self.0.lock().await;

      let name = env!("CARGO_PKG_NAME").to_string();
        let aai = self.2.lock().await;
        aai.track_trace(format!("[REQUEST] {},  {:?}", name, command.clone()), SeverityLevel::Information);

      let customer = lock.create(command.customer.into()).await.expect("Failed to create customer");
      
      aai.track_trace(format!("[RESPONSE] {}, CreateCustomerCommand {:?}", name, customer.clone()), SeverityLevel::Information);

      customer.into()  
    }
}