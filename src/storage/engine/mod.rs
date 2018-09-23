use std::result;
use super::{Key,Value};

pub mod sample_engine;
pub mod data_pool;

pub type Result<T> = result::Result<T, ResultError>;


pub enum ResultError {
    EmptyResult(),
    Overflow(),
}

pub trait Engine {
    //    fn get(&self,key:Key)->Result<Option<Value>>;
    fn get(&self, key: Key) -> Result<Option<Value>>;
     fn put(& self, key: Key, value: Value) -> Result<()>;
     fn delete(& self, key: Key) -> Result<()>;
}

