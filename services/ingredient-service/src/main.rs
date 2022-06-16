mod infra;
mod routes;
mod app;
mod domain;

use std::sync::Arc;

use crate::infra::*;
use crate::routes::*;
use crate::domain::*;

use common::webhost::WebHostBuilder;
use hyper::{Error};
use tokio::sync::Mutex;


#[tokio::main]
async fn main() -> Result<(), Error> {
  
  let setting = infra::Settings::build().unwrap();
  let repository = create_repository::<IngredientCategory>(setting.clone());

     let mediator = create_mediator::<IngredientCategory>(&repository);
     let apidoc = create_api_doc();
     let router = router();

     let host = WebHostBuilder::new(router).build();
     host
     .add_repository(repository)
     .add_mediator(Arc::new(Mutex::new(mediator)))
     .add_apidocs(apidoc.clone())
     .start().await;
     Ok(())

}
