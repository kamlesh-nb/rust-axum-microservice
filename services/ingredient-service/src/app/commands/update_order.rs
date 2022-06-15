use crate::{SharedCosmosRepository, app::OrderDto, domain::Order};
use common::data::Repository;
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderCommand {
  pub id: String,
  pub order: OrderDto,
}

impl Request<OrderDto> for UpdateOrderCommand {}

pub struct UpdateOrderCommandHandler(pub SharedCosmosRepository<Order>, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateOrderCommand, OrderDto> for UpdateOrderCommandHandler {
    async fn handle(&mut self, command: UpdateOrderCommand) -> OrderDto {
      let lock = self.0.lock().await;
      let order = lock.update(command.order.into(), command.id.clone()).await.expect("Failed to update customer");
      order.into()
    }
}