use crate::{SharedCosmosService, app::Order};
use mediator::{AsyncRequestHandler, DefaultAsyncMediator, Request};
use serde::{Deserialize, Serialize};
 

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeleteOrderCommand {
  pub order: Order,
}

impl Request<Order> for DeleteOrderCommand {}

pub struct DeleteOrderCommandHandler(pub SharedCosmosService, pub DefaultAsyncMediator);

#[mediator::async_trait]
impl AsyncRequestHandler<DeleteOrderCommand, Order> for DeleteOrderCommandHandler {
    async fn handle(&mut self, command: DeleteOrderCommand) -> Order {
      let lock = self.0.lock().await;
      let order = lock.delete_order(command.order).await.expect("Failed to delete order");
      order
    }
}