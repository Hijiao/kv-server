use std::result;

pub mod sampledb;
pub mod data_pool;

type Result<T> = result::Result<T, ResultError>;


pub enum ResultError {
    EmptyResult(),
    Overflow(),
}
pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

pub trait Engine {
    //    fn get(&self,key:Key)->Result<Option<Value>>;
     fn get(&self, key: &Key) -> Result<Option<Value>>;
     fn put(&self, key: Key, value: Value) -> Result<()>;
     fn delete(&self, key: Key) -> Result<()>;
}

