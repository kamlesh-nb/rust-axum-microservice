mod settings;
use std::sync::Arc;

use azure_data_cosmos::CosmosEntity;
use common::data::Cosmos;
use mediator::DefaultAsyncMediator;
use serde::{Serialize, de::DeserializeOwned};
pub use settings::*;

mod apidocs;
pub use apidocs::*;
use tokio::sync::Mutex;

use crate::{app::{GetAllOrdersRequestHandler, GetOrderByIdRequestHandler, CreateOrderCommandHandler, DeleteOrderCommandHandler, UpdateOrderCommandHandler}, domain::Order};


pub type SharedMediator = Arc<Mutex<DefaultAsyncMediator>>;
pub type SharedCosmosRepository<V> = Arc<Mutex<Cosmos<V>>>;


pub fn create_repository<V>(setting: Settings) -> SharedCosmosRepository<V> {
  let cosmos = Cosmos::<V>::new(setting.database.key, setting.database.account, setting.database.db, setting.database.container);
  Arc::new(Mutex::new(cosmos))
}


pub fn create_mediator<V>(service: &SharedCosmosRepository<Order>) -> DefaultAsyncMediator  
where V: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send
{
    let mediator = DefaultAsyncMediator::builder()
        .add_handler(GetAllOrdersRequestHandler(service.clone()))
        .add_handler(GetOrderByIdRequestHandler(service.clone()))
        .add_handler_deferred(|m| CreateOrderCommandHandler(service.clone(), m))
        .add_handler_deferred(|m| DeleteOrderCommandHandler(service.clone(), m))
        .add_handler_deferred(|m| UpdateOrderCommandHandler(service.clone(), m))
        .build();
    
        mediator
}

 
 
