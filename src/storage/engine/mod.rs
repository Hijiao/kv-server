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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Task {
    Put(Key, Value),
    Delete(Key),
}

impl Display for Task {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match *self {
            Task::Put(ref k, ref v) => write!(f, "put -> {:?} : {:?}", String::from_utf8(k.clone()).unwrap(), String::from_utf8(v.clone()).unwrap()),
            Task::Delete(ref k) => write!(f, "delete -> {:?}", String::from_utf8(k.clone()).unwrap())
        }
    }
}

pub trait Engine {
    //    fn get(&self,key:Key)->Result<Option<Value>>;
    fn get(&self, key: Key) -> Result<Option<Value>>;
    fn put(&self, key: Key, value: Value) -> Result<()>;
    fn delete(&self, key: Key) -> Result<()>;
    fn find_next(&self, key: Key, next: bool) -> Result<Option<(Key, Value)>>;

}

