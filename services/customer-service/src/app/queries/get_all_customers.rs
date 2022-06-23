use crate::{app::CustomerDto};
use crate::domain::Customer;
use crate::{SharedCosmosRepository, SharedTelemetry};
use appinsights::telemetry::SeverityLevel;
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

#[derive(Debug, Clone)]
pub struct GetAllCustomersRequest{
}

impl Request<Vec<CustomerDto>> for GetAllCustomersRequest {}

pub struct GetAllCustomersRequestHandler(pub SharedCosmosRepository<Customer>, pub SharedTelemetry);

#[mediator::async_trait]
impl AsyncRequestHandler<GetAllCustomersRequest, Vec<CustomerDto>> for GetAllCustomersRequestHandler {

    async fn handle(&mut self, req: GetAllCustomersRequest) -> Vec<CustomerDto> {
        let lock = self.0.lock().await;

        let name = env!("CARGO_PKG_NAME").to_string();
        let aai = self.1.lock().await;
        aai.track_trace(format!("[REQUEST] {},  {:?}", name, req.clone()), SeverityLevel::Information);

        let customers = lock.find_all().await.expect("no customer found");
        let mut customer_dtos: Vec<CustomerDto> = Vec::new();
        for customer in customers {
          customer_dtos.push(customer.into())
        }
        aai.track_trace(format!("[RESPONSE] {}, GetAllCustomersRequest {:?}", name, customer_dtos.clone()), SeverityLevel::Information);
        customer_dtos
    }
}