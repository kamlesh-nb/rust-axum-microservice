use crate::domain::Customer;
use crate::{app::CustomerDto};
use crate::infra::SharedCosmosService;
use mediator::{AsyncRequestHandler, Request};

pub struct GetCustomerByIdRequest{
  pub customer_id: String
}

impl Request<Vec<CustomerDto>> for GetCustomerByIdRequest {}

pub struct GetCustomerByIdRequestHandler(pub SharedCosmosService<Customer>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetCustomerByIdRequest, Vec<CustomerDto>> for GetCustomerByIdRequestHandler {

    async fn handle(&mut self, req: GetCustomerByIdRequest) -> Vec<CustomerDto> {
        let lock = self.0.lock().await;
        let customers = lock.get_by_id(req.customer_id).await.expect("no customer found");
        let mut customer_dtos: Vec<CustomerDto> = Vec::new();
        for customer in customers {
          customer_dtos.push(customer.into())
        }
        customer_dtos
    }
}