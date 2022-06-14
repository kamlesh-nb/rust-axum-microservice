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

#[derive(Debug, Clone)]
pub struct CosmosService<V> {
    key: String,
    account: String,
    db: String,
    container: String,
    _marker: PhantomData<V>,
}

#[allow(unused)]
impl<V> CosmosService<V>
where V: Serialize + DeserializeOwned + Clone + CosmosEntity + 'static + Send {
    
  pub fn new(key: String, account: String, db: String, container: String) -> Self {
    Self{
      key, 
      account, 
      db, 
      container,
      _marker: PhantomData
    }
  }

  async fn create_client(&self) -> CollectionClient {
    let authorization_token = AuthorizationToken::primary_from_base64(&self.key).unwrap();
    let client = CosmosClient::new(
        self.account.clone(),
        authorization_token,
        CosmosOptions::default(),
    );

    let database = client.database_client(self.db.clone());
    database.collection_client(self.container.clone())
  }

  pub async fn get_all(&self) -> Result<Vec<V>, Box<dyn Error + Send + Sync>> {
    let query = format!("SELECT * FROM c");
      let query_obj = Query::new(query);
      let client = self.create_client().await;
      let query_result = client
          .query_documents(query_obj)
          .query_cross_partition(true)
          .into_stream::<V>()
          .next()
          .await
          .unwrap()
          .unwrap()
          .into_documents()
          .unwrap()
          .results;

      let mut rows: Vec<V> = Vec::new();
      for document in query_result {
          rows.push(document.result)
      }
      Ok(rows)
  }

  pub async fn get_by_id(&self, id: String) -> Result<Vec<V>, Box<dyn Error + Send + Sync>> {
    let query = format!("SELECT * FROM c where id='{}'", id);
      let query_obj = Query::new(query);
      let client = self.create_client().await;
      let query_result = client
          .query_documents(query_obj)
          .query_cross_partition(true)
          .into_stream::<V>()
          .next()
          .await
          .unwrap()
          .unwrap()
          .into_documents()
          .unwrap()
          .results;

      let mut rows: Vec<V> = Vec::new();
      for document in query_result {
          rows.push(document.result)
      }
      Ok(rows)
  }

  pub async fn get_by_query(&self, query: String) -> Result<Vec<V>, Box<dyn Error + Send + Sync>> {
      let query_obj = Query::new(query);
      let client = self.create_client().await;
      let query_result = client
          .query_documents(query_obj)
          .query_cross_partition(true)
          .into_stream::<V>()
          .next()
          .await
          .unwrap()
          .unwrap()
          .into_documents()
          .unwrap()
          .results;

      let mut rows: Vec<V> = Vec::new();
      for document in query_result {
          rows.push(document.result)
      }
      Ok(rows)
  }

  pub async fn create(&self, doc: V) -> Result<V, Box<dyn Error + Send + Sync>> {
    
      let client = self.create_client().await;
      client
          .create_document(doc.clone())
          .into_future()
          .await?;
      Ok(doc)
  }

  pub async fn update(&self, doc: V, id: String) -> Result<V, Box<dyn Error + Send + Sync>> {

    let client = self.create_client().await;
    client
        .document_client(id.clone(), &id)
        .unwrap()
        .replace_document(doc.clone())
        .into_future()
        .await
        .unwrap();

    Ok(doc)
}

  pub async fn delete(&self, id: String) -> Result<(), Box<dyn Error + Send + Sync>> {

    let client = self.create_client().await;
    client
        .document_client(id.clone(), &id)
        .unwrap()
        .delete_document()
        .into_future()
        .await
        .unwrap();

    Ok(())
  }

}