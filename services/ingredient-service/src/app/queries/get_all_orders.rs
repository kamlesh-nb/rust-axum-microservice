use crate::{app::OrderDto};
use crate::domain::Order;
use crate::{SharedCosmosRepository};
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetAllOrdersRequest{
}

impl Request<Vec<OrderDto>> for GetAllOrdersRequest {}

pub struct GetAllOrdersRequestHandler(pub SharedCosmosRepository<Order>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetAllOrdersRequest, Vec<OrderDto>> for GetAllOrdersRequestHandler {

    async fn handle(&mut self, _req: GetAllOrdersRequest) -> Vec<OrderDto> {
        let lock = self.0.lock().await;
        let customers = lock.find_all().await.expect("no customer found");
        let mut customer_dtos: Vec<OrderDto> = Vec::new();
        for customer in customers {
          customer_dtos.push(customer.into())
        }
        customer_dtos
    }
}