use crate::{SharedCosmosRepository, app::OrderDto, domain::Order};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderCommand {
  pub order: OrderDto,
}

impl Request<OrderDto> for CreateOrderCommand {}

pub struct CreateOrderCommandHandler(pub SharedCosmosRepository<Order>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateOrderCommand, OrderDto> for CreateOrderCommandHandler {
    async fn handle(&mut self, command: CreateOrderCommand) -> OrderDto {
      let lock = self.0.lock().await;
      let order = lock.create(command.order.into()).await.expect("Failed to create customer");
      order.into()  
    }
}