use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct Context<T> {
   pub uri: String,
   pub(crate) _marker: PhantomData<T>
}