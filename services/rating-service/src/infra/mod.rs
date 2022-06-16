use std::sync::Arc;

use azure_data_cosmos::CosmosEntity;
use common::data::Cosmos;
// use common::{cosmosdb::CosmosService, data::Cosmos};
use mediator::DefaultAsyncMediator;

mod settings;
pub use settings::*;

mod apidoc;
pub use apidoc::*;

use crate::{app::*, domain::Rating};

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::Mutex;


pub type SharedMediator = Arc<Mutex<DefaultAsyncMediator>>;
pub type SharedCosmosRepository<V> = Arc<Mutex<Cosmos<V>>>;


pub fn create_repository<V>(setting: Settings) -> Cosmos<V> 
where 
V: Serialize + DeserializeOwned + CosmosEntity + Clone + 'static + Send + Sync  
{
  let cosmos = Cosmos::<V>::new(setting.database.key, setting.database.account, setting.database.db, setting.database.container);
  cosmos
}

pub fn create_mediator<V>(service: &SharedCosmosRepository<Rating>) -> DefaultAsyncMediator  
where V: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send
{
  let mediator = DefaultAsyncMediator::builder()
        .add_handler(GetAllRatingsRequestHandler(service.clone()))
        .add_handler(GetRatingByIdRequestHandler(service.clone()))
        .add_handler_deferred(|m| CreateRatingCommandHandler(service.clone(), m))
        .add_handler_deferred(|m| DeleteRatingCommandHandler(service.clone(), m))
        .add_handler_deferred(|m| UpdateRatingCommandHandler(service.clone(), m))
        .build();
    
        mediator
}