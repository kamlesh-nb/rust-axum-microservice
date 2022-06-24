mod infra;
mod routes;
mod app;
mod domain;

use std::sync::Arc;

use crate::infra::*;
use crate::domain::*;

use appinsights::TelemetryClient;
use common::webhost::WebHostBuilder;
use hyper::{Error};
use tokio::sync::Mutex;


#[tokio::main]
async fn main() -> Result<(), Error> {
  let name = env!("CARGO_PKG_NAME").to_string();
  let setting = infra::Settings::build().unwrap();
  let repository = create_repository::<Order>(setting.clone());
  let logger = create_logger(setting.appinsight.key.clone());  
     let mediator = create_mediator::<Order>(&Arc::new(Mutex::new(repository.clone())));
     let apidoc = create_api_doc();
     let app = routes::router();

     let host = WebHostBuilder::new(app, name.clone()).build();
     host
     .add_repository(repository)
     .add_logger::<TelemetryClient>(logger)
     .add_mediator(Arc::new(Mutex::new(mediator)))
     .add_apidocs(apidoc.clone())
     .start().await;
     Ok(())

}
