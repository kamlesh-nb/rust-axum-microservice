use crate::domain::Customer;
use crate::{app::CustomerDto};
use crate::infra::{SharedCosmosRepository, SharedTelemetry};
use appinsights::telemetry::SeverityLevel;
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};
use serde::{Serialize, Deserialize};
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GetCustomerByIdRequest{
  pub id: String
}

impl Request<Vec<CustomerDto>> for GetCustomerByIdRequest {}

pub struct GetCustomerByIdRequestHandler(pub SharedCosmosRepository<Customer>, pub SharedTelemetry);

#[mediator::async_trait]
impl AsyncRequestHandler<GetCustomerByIdRequest, Vec<CustomerDto>> for GetCustomerByIdRequestHandler {

    async fn handle(&mut self, req: GetCustomerByIdRequest) -> Vec<CustomerDto> {
        
        let lock = self.0.lock().await;

        let name = env!("CARGO_PKG_NAME").to_string();
        let aai = self.1.lock().await;
        aai.track_trace(format!("[REQUEST] {},  {:?}", name, req.clone()), SeverityLevel::Information);
        
        let customers = lock.find_by_id(req.id).await.expect("no customer found");
        let mut customer_dtos: Vec<CustomerDto> = Vec::new();
        for customer in customers {
          customer_dtos.push(customer.into())
        }

        aai.track_trace(format!("[RESPONSE] {}, GetCustomerByIdRequest {:?}", name, customer_dtos.clone()), SeverityLevel::Information);
        customer_dtos
    }
}