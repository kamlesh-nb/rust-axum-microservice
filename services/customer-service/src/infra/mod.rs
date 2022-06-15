use std::sync::Arc;

use azure_data_cosmos::CosmosEntity;
use common::data::Cosmos;
// use common::{cosmosdb::CosmosService, data::Cosmos};
use mediator::DefaultAsyncMediator;

mod settings;
pub use settings::*;

mod apidoc;
pub use apidoc::*;

use crate::{app::*, domain::Customer};

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::Mutex;

// pub type SharedCosmosService<V> = Arc<Mutex<CosmosService<V>>>;
pub type SharedMediator = Arc<Mutex<DefaultAsyncMediator>>;
pub type SharedCosmosRepository<V> = Arc<Mutex<Cosmos<V>>>;

// pub fn create_cosmosdb<V>(setting: Settings) -> SharedCosmosService<V> 
// where V: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send
// {
//   let service = CosmosService::new(setting.database.key, setting.database.account, setting.database.db, setting.database.container);
//   Arc::new(Mutex::new(service))
// }


pub fn create_repository<V>(setting: Settings) -> SharedCosmosRepository<V> {
  let cosmos = Cosmos::<V>::new(setting.database.key, setting.database.account, setting.database.db, setting.database.container);
  Arc::new(Mutex::new(cosmos))
}

pub fn create_mediator<V>(service: &SharedCosmosRepository<Customer>) -> DefaultAsyncMediator  
where V: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send
{
  let mediator = DefaultAsyncMediator::builder()
        .add_handler(GetAllCustomersRequestHandler(service.clone()))
        .add_handler(GetCustomerByIdRequestHandler(service.clone()))
        .add_handler_deferred(|m| CreateCustomerCommandHandler(service.clone(), m))
        .add_handler_deferred(|m| DeleteCustomerCommandHandler(service.clone(), m))
        .add_handler_deferred(|m| UpdateCustomerCommandHandler(service.clone(), m))
        .build();
    
        mediator
}