use crate::{SharedCosmosService, app::Order};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateOrderCommand {
  pub order: Order,
}

impl Request<Order> for CreateOrderCommand {}

pub struct CreateOrderCommandHandler(pub SharedCosmosService, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<CreateOrderCommand, Order> for CreateOrderCommandHandler {
    async fn handle(&mut self, command: CreateOrderCommand) -> Order {
      let lock = self.0.lock().await;
      let order = lock.create_order(command.order).await.expect("Failed to create order");
      order
    }
}