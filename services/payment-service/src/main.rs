use std::sync::Arc;

 
use appinsights::TelemetryClient;
use common::webhost;

use hyper::Error;
use tokio::sync::Mutex;

mod routes;
mod infra;
mod domain;
mod app;

use crate::infra::*;
use crate::domain::*;

 
#[tokio::main]
async fn main() -> Result<(), Error> {
  let name = env!("CARGO_PKG_NAME").to_string();
  let setting = infra::Settings::build().unwrap();
  let repository = create_repository::<Payment>(setting.clone());
  let logger = create_logger(setting.appinsight.key.clone());  
  let mediatr = create_mediator::<Payment>(&Arc::new(Mutex::new(repository.clone())));
  let apidoc = create_api_doc();
  let app = routes::router();
  let host = webhost::WebHostBuilder::new(app, name.clone()).build();
  host
  .add_settings(Arc::new(Mutex::new(setting.clone())))
  .add_logger::<TelemetryClient>(logger)
  .add_repository(repository)
  .add_mediator(Arc::new(Mutex::new(mediatr)))
  .add_apidocs(apidoc)
  .start().await;
  Ok(())
}
