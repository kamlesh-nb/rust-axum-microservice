use std::sync::Arc;

 
use common::webhost;
use common::data::*;

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
  let repository = Context::<Customer>::new("kjhkjh".to_string());
  let r = repository.find_all().await.unwrap();

  let setting = infra::Settings::build().unwrap();
  println!("{:?}", setting.clone());
  let cosmos = create_cosmosdb::<Customer>(setting.clone());
  let mediatr = create_mediator::<Customer>(&cosmos);
  let apidoc = create_api_doc();
  let app = routes::router();
  let host = webhost::WebHostBuilder::new(app).build();
  host
  .add_settings(Arc::new(Mutex::new(setting.clone())))
  .add_cosmosdb(cosmos)
  .add_repository(Arc::new(Mutex::new(repository)))
  .add_mediator(Arc::new(Mutex::new(mediatr)))
  .add_apidocs(apidoc)
  .start().await;
  Ok(())
}
