use crate::{app::Order};
use crate::{SharedCosmosService};
use mediator::{AsyncRequestHandler, Request};

pub struct GetOrderByIdRequest{
  pub order_id: String
}

impl Request<Vec<Order>> for GetOrderByIdRequest {}

pub struct GetOrderByIdRequestHandler(pub SharedCosmosService);

#[mediator::async_trait]
impl AsyncRequestHandler<GetOrderByIdRequest, Vec<Order>> for GetOrderByIdRequestHandler {

    async fn handle(&mut self, req: GetOrderByIdRequest) -> Vec<Order> {
        let lock = self.0.lock().await;
        let orders = lock.get_order_by_id(req.order_id).await.expect("no order found");
        orders
    }
}