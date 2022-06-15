use crate::domain::Order;
use crate::{app::OrderDto};
use crate::infra::SharedCosmosRepository;
use common::data::Repository;
use mediator::{AsyncRequestHandler, Request};

pub struct GetOrderByIdRequest{
  pub id: String
}

impl Request<Vec<OrderDto>> for GetOrderByIdRequest {}

pub struct GetOrderByIdRequestHandler(pub SharedCosmosRepository<Order>);

#[mediator::async_trait]
impl AsyncRequestHandler<GetOrderByIdRequest, Vec<OrderDto>> for GetOrderByIdRequestHandler {

    async fn handle(&mut self, req: GetOrderByIdRequest) -> Vec<OrderDto> {
        let lock = self.0.lock().await;
        let customers = lock.find_by_id(req.id).await.expect("no customer found");
        let mut customer_dtos: Vec<OrderDto> = Vec::new();
        for customer in customers {
          customer_dtos.push(customer.into())
        }
        customer_dtos
    }
}