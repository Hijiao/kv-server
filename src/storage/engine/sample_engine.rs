use super::data_pool::DataPool;
use super::Engine;
use super::{Key, Value, Result};

struct SampleEngine {
    data_pool: DataPool,
}


impl SampleEngine {
    fn new() -> Self {
        SampleEngine {
            data_pool: DataPool::new()
        }
    }
}

impl Engine for SampleEngine {
    fn get(&self, key: Key) -> Result<Option<Value>> {
        let ret = self.data_pool.get(key);
        match ret {
            Some(s) => Ok(Some(s.into_bytes())),
            None => Ok(None)
        }
    }
    fn put(&mut self, key: Key, value: Value) -> Result<()> {
        self.data_pool.insert(key, value);
        Ok(())
    }
    fn delete(&mut self, key: Key) -> Result<()> {
        self.data_pool.delete(key);
        Ok(())
    }
}

#[test]
fn engine_test() {
    let mut engine = SampleEngine::new();
    let k = b"k".to_vec();
    let v = b"v".to_vec();

    let _ = engine.put(k.clone(), v);

    let v = engine.get(k.clone()).ok().unwrap().unwrap();

    assert_eq!("v", unsafe { String::from_utf8_unchecked(v) });

    let _ = engine.delete(k.clone());

    let ret = engine.get(k.clone());

    assert_eq!(Some(None), ret.ok());
}