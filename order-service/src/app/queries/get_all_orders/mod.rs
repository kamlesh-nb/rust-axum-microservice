use crate::{app::Order};
use crate::{SharedCosmosService};
use mediator::{AsyncRequestHandler, Request};

pub struct GetAllOrdersRequest{
}

impl Request<Vec<Order>> for GetAllOrdersRequest {}

pub struct GetAllOrdersRequestHandler(pub SharedCosmosService);

#[mediator::async_trait]
impl AsyncRequestHandler<GetAllOrdersRequest, Vec<Order>> for GetAllOrdersRequestHandler {

    async fn handle(&mut self, _req: GetAllOrdersRequest) -> Vec<Order> {
        let lock = self.0.lock().await;
        let orders = lock.get_all_orders().await.expect("no order found");
        orders
    }
}