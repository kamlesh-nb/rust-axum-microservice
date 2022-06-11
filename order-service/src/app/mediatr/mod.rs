use std::sync::Arc;
use mediator::DefaultAsyncMediator;
use tokio::sync::Mutex;
use crate::infra::SharedCosmosService;


use crate::app::queries::*;
use crate::app::commands::*;

pub type SharedMediator = Arc<Mutex<DefaultAsyncMediator>>;

pub fn create_mediator(cosmos: &SharedCosmosService) -> DefaultAsyncMediator {
    let mediator = DefaultAsyncMediator::builder()
        .add_handler(GetAllOrdersRequestHandler(cosmos.clone()))
        .add_handler(GetOrderByIdRequestHandler(cosmos.clone()))
        .add_handler_deferred(|m| CreateOrderCommandHandler(cosmos.clone(), m))
        .add_handler_deferred(|m| DeleteOrderCommandHandler(cosmos.clone(), m))
        .add_handler_deferred(|m| UpdateOrderCommandHandler(cosmos.clone(), m))
        .build();
    
        mediator
}