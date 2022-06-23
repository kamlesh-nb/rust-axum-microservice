use std::sync::Arc;
use std::time::Duration;

use appinsights::TelemetryClient;
use appinsights::TelemetryConfig;

use azure_data_cosmos::CosmosEntity;
use common::data::Cosmos;
use mediator::DefaultAsyncMediator;

mod settings;
pub use settings::*;

mod apidoc;
pub use apidoc::*;

use crate::{app::*, domain::Customer};

use serde::{Serialize, de::DeserializeOwned};
use tokio::sync::Mutex;

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

pub fn create_mediator<V>(service: &SharedCosmosRepository<Customer>, logger: SharedTelemetry) -> DefaultAsyncMediator  
where V: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send
{
  let mediator = DefaultAsyncMediator::builder()
        .add_handler(GetAllCustomersRequestHandler(service.clone(), logger.clone()))
        .add_handler(GetCustomerByIdRequestHandler(service.clone(),logger.clone()))
        .add_handler_deferred(|m| CreateCustomerCommandHandler(service.clone(), m, logger.clone()))
        .add_handler_deferred(|m| DeleteCustomerCommandHandler(service.clone(), m, logger.clone()))
        .add_handler_deferred(|m| UpdateCustomerCommandHandler(service.clone(), m, logger.clone()))
        .build();
    
        mediator
}

 
