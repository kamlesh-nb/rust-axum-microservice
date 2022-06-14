use serde::de::DeserializeOwned;
use serde::Serialize;
use std::marker::PhantomData;
use futures::StreamExt;
use std::error::Error;
use azure_data_cosmos::{
  clients::{CollectionClient, CosmosClient, CosmosOptions},
  prelude::{AuthorizationToken, Query},
  CosmosEntity
};

use super::{Repository, Context};

impl Repository<T> for Context<T> 
where T: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send
{
    fn create(&self, value: T) -> Result<T, Box<dyn Error + Send + Sync>> {
        todo!()
    }

    fn update(&self, value: T, filter: String) -> Result<T, Box<dyn Error + Send + Sync>> {
        todo!()
    }

    fn delete(&self, value: T, filter: String) -> Result<T, Box<dyn Error + Send + Sync>> {
        todo!()
    }

    fn find_all(&self) -> Result<Vec<T>, Box<dyn Error + Send + Sync>> {
        todo!()
    }

    fn find_by_id(&self, id: String) -> Result<Vec<T>, Box<dyn Error + Send + Sync>> {
        todo!()
    }

    fn find_by_query(&self, query: String) -> Result<Vec<T>, Box<dyn Error + Send + Sync>> {
        todo!()
    }
}