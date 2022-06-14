use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Context<V> {
   uri: String,
   _marker: PhantomData<V>,
}