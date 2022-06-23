use crate::{SharedCosmosRepository, SharedTelemetry, domain::Customer};
use appinsights::telemetry::SeverityLevel;
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteCustomerCommand {
  pub id: String,
}

impl Request<String> for DeleteCustomerCommand {}

pub struct DeleteCustomerCommandHandler(pub SharedCosmosRepository<Customer>, pub DefaultAsyncMediator, pub SharedTelemetry);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteCustomerCommand, String> for DeleteCustomerCommandHandler {
    async fn handle(&mut self, command: DeleteCustomerCommand) -> String {
      let lock = self.0.lock().await;

      let name = env!("CARGO_PKG_NAME").to_string();
        let aai = self.2.lock().await;
        aai.track_trace(format!("[REQUEST] {},  {:?}", name, command.clone()), SeverityLevel::Information);

      let _ = lock.delete(command.id.clone()).await.expect("Failed to delete customer");
      
      aai.track_trace(format!("[RESPONSE] {}, DeleteCustomerCommand {:?}", name, command.clone()), SeverityLevel::Information);

      command.id
    }
}