use azure_data_cosmos::{
    clients::{CollectionClient, CosmosClient, CosmosOptions},
    prelude::{AuthorizationToken, Query},
};
use mediator::futures::StreamExt;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::Mutex;

use crate::{app::Order, domain};

use super::Settings;

#[derive(Debug, Clone)]
pub struct CosmosService {
    pub key: String,
    pub account: String,
    pub db: String,
    pub container: String,
}

impl CosmosService {
    pub fn new(key: String, account: String, db: String, container: String) -> Self {
        Self {
            key,
            account,
            db,
            container,
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

    pub async fn get_all_orders(&self) -> Result<Vec<Order>, Box<dyn Error + Send + Sync>> {
        let query = format!("SELECT * FROM c");
        let query_obj = Query::new(query);
        let client = self.create_client().await;
        let query_result = client
            .query_documents(query_obj)
            .query_cross_partition(true)
            .into_stream::<domain::Order>()
            .next()
            .await
            .unwrap()
            .unwrap()
            .into_documents()
            .unwrap()
            .results;

        let mut orders: Vec<Order> = Vec::new();
        for document in query_result {
            orders.push(document.result.into())
        }
        Ok(orders)
    }

    pub async fn get_order_by_id(&self, id: String) -> Result<Vec<Order>, Box<dyn Error + Send + Sync>> {
        let query = format!("SELECT * FROM c where c.id='{}'", id);
        let query_obj = Query::new(query);
        let client = self.create_client().await;
        let query_result = client
            .query_documents(query_obj)
            .query_cross_partition(true)
            .into_stream::<domain::Order>()
            .next()
            .await
            .unwrap()
            .unwrap()
            .into_documents()
            .unwrap()
            .results;

        let mut orders: Vec<Order> = Vec::new();
        for document in query_result {
            orders.push(document.result.into())
        }
        Ok(orders)
    }

    pub async fn create_order(&self, order: Order) -> Result<Order, Box<dyn Error + Send + Sync>> {
       let mut c_order = domain::Order::from(order.clone());

       c_order.created_by = Some("kamlesh".to_string());
       c_order.created_on = Some(chrono::Utc::now());

        let client = self.create_client().await;
        client
            .create_document(c_order.clone())
            .is_upsert(false)
            .into_future()
            .await?;
        Ok(order)
    }

    pub async fn update_order(&self, order: Order) -> Result<Order, Box<dyn Error + Send + Sync>> {
      let mut c_order = domain::Order::from(order.clone());

       c_order.created_by = Some("kamlesh".to_string());
       c_order.created_on = Some(chrono::Utc::now());

        let client = self.create_client().await;
        client
            .document_client(c_order.id.clone(), &c_order.id)
            .unwrap()
            .replace_document(c_order)
            .into_future()
            .await
            .unwrap();

        Ok(order)
    }

    pub async fn delete_order(&self, order: Order) -> Result<Order, Box<dyn Error + Send + Sync>> {
      let mut c_order = domain::Order::from(order.clone());

       c_order.created_by = Some("kamlesh".to_string());
       c_order.created_on = Some(chrono::Utc::now());

       let client = self.create_client().await;
       client
           .document_client(c_order.id.clone(), &c_order.id)
           .unwrap()
           .delete_document()
           .into_future()
           .await
           .unwrap();

        Ok(order)
    }

}

pub type SharedCosmosService = Arc<Mutex<CosmosService>>;

pub fn create_cosmos(setting: Settings) -> SharedCosmosService {
    let service = CosmosService::new(
        setting.database.key,
        setting.database.account,
        setting.database.db,
        setting.database.container,
    );
    Arc::new(Mutex::new(service))
}
