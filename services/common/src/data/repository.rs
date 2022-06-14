use std::error::Error;


pub trait Repository<T> {
  fn create(&self, value: T) -> Result<T, Box<dyn Error + Send + Sync>>;
  fn update(&self, value: T, filter: String) -> Result<T, Box<dyn Error + Send + Sync>>;
  fn delete(&self, value: T, filter: String) -> Result<T, Box<dyn Error + Send + Sync>>;
  fn find_all(&self) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;
  fn find_by_id(&self, id: String) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;
  fn find_by_query(&self, query: String) -> Result<Vec<T>, Box<dyn Error + Send + Sync>>;
}