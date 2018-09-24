use std::result;
use super::{Key, Value};
use std::fmt::{self, Display, Formatter};

pub mod sample_engine;
pub mod data_pool;
pub mod data_writer;

pub type Result<T> = result::Result<T, ResultError>;


pub enum ResultError {
//    EmptyResult(),
//    Overflow(),
}

#[derive(Debug)]
pub enum Task {
    Delete(Key),
    Put(Key, Value),
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Task::Delete(..) => write!(f, "delete task"),
            Task::Put(..) => write!(f, "put  task"),
        }
    }
}

pub trait Engine {
    //    fn get(&self,key:Key)->Result<Option<Value>>;
    fn get(&self, key: Key) -> Result<Option<Value>>;
    fn put(&self, key: Key, value: Value) -> Result<()>;
    fn delete(&self, key: Key) -> Result<()>;
}

