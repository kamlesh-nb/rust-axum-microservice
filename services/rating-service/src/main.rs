use std::sync::Arc;

 
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
  
  let setting = infra::Settings::build().unwrap();
  let repository = create_repository::<Rating>(setting.clone());

  let mediatr = create_mediator::<Rating>(&repository);
  let apidoc = create_api_doc();
  let app = routes::router();
  let host = webhost::WebHostBuilder::new(app).build();
  host
  .add_settings(Arc::new(Mutex::new(setting.clone())))
  .add_repository(repository)
  .add_mediator(Arc::new(Mutex::new(mediatr)))
  .add_apidocs(apidoc)
  .start().await;
  Ok(())
}
