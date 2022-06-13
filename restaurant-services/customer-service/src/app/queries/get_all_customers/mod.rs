use crate::{app::CustomerDto};
use crate::domain::Customer;
use crate::{SharedCosmosService};
use mediator::{AsyncRequestHandler, Request};

pub struct GetAllCustomersRequest{
}

impl Request<Vec<CustomerDto>> for GetAllCustomersRequest {}

pub struct GetAllCustomersRequestHandler(pub SharedCosmosService<Customer>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetAllCustomersRequest, Vec<CustomerDto>> for GetAllCustomersRequestHandler {

    async fn handle(&mut self, _req: GetAllCustomersRequest) -> Vec<CustomerDto> {
        let lock = self.0.lock().await;
        let customers = lock.get_all().await.expect("no customer found");
        customers
    }
}