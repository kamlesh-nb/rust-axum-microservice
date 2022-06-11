use crate::{SharedCosmosService, app::Order};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateOrderCommand {
  pub order: Order,
}

impl Request<Order> for UpdateOrderCommand {}

pub struct UpdateOrderCommandHandler(pub SharedCosmosService, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<UpdateOrderCommand, Order> for UpdateOrderCommandHandler {
    async fn handle(&mut self, command: UpdateOrderCommand) -> Order {
      let lock = self.0.lock().await;
      let order = lock.update_order(command.order).await.expect("Failed to update order");
      order
    }
}