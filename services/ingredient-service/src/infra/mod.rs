mod settings;
use std::{sync::Arc, time::Duration};

use appinsights::{TelemetryClient, TelemetryConfig};
use azure_data_cosmos::CosmosEntity;
use common::data::{Cosmos};
use mediator::DefaultAsyncMediator;
use serde::{Serialize, de::DeserializeOwned};
pub use settings::*;

mod apidocs;
pub use apidocs::*;
use tokio::sync::Mutex;

use crate::{app::{GetAllIngredientCategorysRequestHandler, GetIngredientCategoryByIdRequestHandler, CreateIngredientCategoryCommandHandler, DeleteIngredientCategoryCommandHandler, UpdateIngredientCategoryCommandHandler}, domain::IngredientCategory};


pub type SharedMediator = Arc<Mutex<DefaultAsyncMediator>>;
pub type SharedCosmosRepository<V> = Arc<Mutex<Cosmos<V>>>;
pub type SharedTelemetry = Arc<Mutex<TelemetryClient>>;

pub fn create_logger(key: String) -> SharedTelemetry {
  let config = TelemetryConfig::builder()
      .i_key(key)
      .interval(Duration::from_secs(5))
      .build();
  
  let client = TelemetryClient::from_config(config);
  Arc::new(Mutex::new(client))
}

pub fn create_repository<V>(setting: Settings) -> Cosmos<V> 
where 
V: Serialize + DeserializeOwned + CosmosEntity + Clone + 'static + Send + Sync  
{
  let cosmos = Cosmos::<V>::new(setting.database.key, setting.database.account, setting.database.db, setting.database.container);
  cosmos
}

pub fn create_mediator<V>(service: &SharedCosmosRepository<IngredientCategory>) -> DefaultAsyncMediator  
where V: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send
{
    let mediator = DefaultAsyncMediator::builder()
        .add_handler(GetAllIngredientCategorysRequestHandler(service.clone()))
        .add_handler(GetIngredientCategoryByIdRequestHandler(service.clone()))
        .add_handler_deferred(|m| CreateIngredientCategoryCommandHandler(service.clone(), m))
        .add_handler_deferred(|m| DeleteIngredientCategoryCommandHandler(service.clone(), m))
        .add_handler_deferred(|m| UpdateIngredientCategoryCommandHandler(service.clone(), m))
        .build();
    
        mediator
}

 
 
