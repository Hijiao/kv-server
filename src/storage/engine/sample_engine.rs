use super::data_pool::DataPool;
use super::Engine;
use super::{Key, Value, Result};
use std::sync::{Arc, RwLock};

#[derive(Clone)]
pub struct SampleEngine {
    data_pool: Arc<RwLock<DataPool>>,

}

impl SampleEngine {
    pub fn new() -> SampleEngine {
        SampleEngine {
            data_pool: Arc::new(RwLock::new(DataPool::new()))
        }
    }
}

impl Engine for SampleEngine {
     fn get(&self, key: Key) -> Result<Option<Value>> {
        let ret = self.data_pool.read().unwrap().get(key);

        match ret {
            Some(s) => Ok(Some(s.into_bytes())),
            None => Ok(None)
        }
    }
    fn put(&self, key: Key, value: Value) -> Result<()> {
        let mut data_pool = self.data_pool.write().unwrap();
        data_pool.insert(key, value);
        Ok(())
    }
    fn delete(&self, key: Key) -> Result<()> {
        let mut data_pool = self.data_pool.write().unwrap();
        data_pool.delete(key);
        Ok(())
    }
}

#[test]
fn engine_test() {
    let  engine = SampleEngine::new();
    let k = b"k".to_vec();
    let v = b"v".to_vec();

    let _ = engine.put(k.clone(), v);

    let v = engine.get(k.clone()).ok().unwrap().unwrap();

    assert_eq!("v", unsafe { String::from_utf8_unchecked(v) });

    let _ = engine.delete(k.clone());

    let ret = engine.get(k.clone());

    assert_eq!(Some(None), ret.ok());
}