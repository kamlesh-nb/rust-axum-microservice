use common::{webhost};
use hyper::Error;

mod routes;
mod infra;
mod domain;
mod app;

use crate::infra::*;
use crate::domain::*;
 
#[tokio::main]
async fn main() -> Result<(), Error> {
  let setting = infra::Settings::new().unwrap();
  let cosmos = create_cosmosdb::<Customer>(setting);
  let app = routes::router();
  let host = webhost::WebHostBuilder::new(app).build();
  host.add_cosmosdb(cosmos).start().await;
  Ok(())
}
