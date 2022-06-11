mod infra;
mod routes;
mod app;
mod domain;

use crate::infra::*;
use crate::routes::*;
use crate::app::*;

use hyper::{Error};


#[tokio::main]
async fn main() -> Result<(), Error> {
  
     let settings = Settings::new().unwrap();
     let service = create_cosmos(settings.clone());
     let mediator = create_mediator(&service);
     let apidoc = create_api_doc();
     let router = router();

     let host = WebHostBuilder::new(router).build();
     host
     .add_cosmosdb(service.clone())
     .add_mediator(mediator)
     .add_api_docs(apidoc.clone())
     .start(settings).await;
     Ok(())

}
