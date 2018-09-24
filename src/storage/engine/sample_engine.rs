use super::data_pool::DataPool;
use super::Engine;
use super::{Key, Value, Result, Task};
use std::sync::{Arc, RwLock};
use super::data_writer::{WriteQueue, DataWriter};


#[derive(Clone)]
pub struct SampleEngine {
    data_pool: Arc<RwLock<DataPool>>,
    write_queue: WriteQueue,
}

impl SampleEngine {
    pub fn new() -> SampleEngine {
        let mut data_write = DataWriter::new();
        data_write.start().ok();

        SampleEngine {
            data_pool: Arc::new(RwLock::new(DataPool::new())),
            write_queue: data_write.get_queue().clone(),
        }
    }
    pub fn shutdown(&self) {
        println!("sample engine shutdown ...");
        self.write_queue.append(None);
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
        self.write_queue.append(Some(Task::Put(key.clone(), value.clone())));
        let mut data_pool = self.data_pool.write().unwrap();
        data_pool.insert(key, value);
        Ok(())
    }
    fn delete(&self, key: Key) -> Result<()> {
        self.write_queue.append(Some(Task::Delete(key.clone())));
        let mut data_pool = self.data_pool.write().unwrap();
        data_pool.delete(key);
        Ok(())
    }
}

#[test]
fn engine_test() {
    use std::thread;
    use std::time::Duration;

    let engine = SampleEngine::new();
    let k = b"k".to_vec();
    let v = b"v".to_vec();

    let _ = engine.put(k.clone(), v);

    let v = engine.get(k.clone()).ok().unwrap().unwrap();

    assert_eq!("v", unsafe { String::from_utf8_unchecked(v) });

    let _ = engine.delete(k.clone());

    let ret = engine.get(k.clone());

    assert_eq!(Some(None), ret.ok());
    thread::sleep(Duration::from_millis(100));

    engine.shutdown();
}