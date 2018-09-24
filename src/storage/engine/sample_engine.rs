use super::data_pool::DataPool;
use super::Engine;
use super::{Key, Value, Result, Task};
use std::sync::{Arc, RwLock};
use super::data_writer::DataWriter;
use std::fs::OpenOptions;
use std::io::BufReader;
use std::io::BufRead;

pub const DEFAULT_LOG_PATH: &'static str = "default-log.txt";

const AUTO_RECOVERY: bool = true;

pub struct SampleEngineBuilder {
    auto_recovery: Option<bool>,
    log_path: Option<String>,
}

impl SampleEngineBuilder {
    pub fn new() -> SampleEngineBuilder {
        SampleEngineBuilder {
            auto_recovery: None,
            log_path: None,
        }
    }
    pub fn auto_recovery(mut self, auto: bool) -> Self {
        self.auto_recovery = Some(auto);
        self
    }
    pub fn set_log_path(mut self, log_path: &str) -> Self {
        self.log_path = Some(log_path.to_string());
        self
    }
    pub fn builder(self) -> SampleEngine {
        let log_path = match self.log_path {
            None => DEFAULT_LOG_PATH.to_string(),
            Some(path) => path,
        };

        let auto_recovery = match self.auto_recovery {
            None => AUTO_RECOVERY.clone(),
            Some(auto) => auto,
        };

        let mut data_writer = DataWriter::new();
        data_writer.log_path = Some(log_path);

        if auto_recovery {
            data_writer.start().ok();
        }

        let sample_engine = SampleEngine {
            data_pool: Arc::new(RwLock::new(DataPool::new())),
            data_writer: Arc::new(data_writer),
            auto_recovery,

        };

        if auto_recovery {
            sample_engine.recovery_from_file();
        }
        sample_engine
    }
}


#[derive(Clone)]
pub struct SampleEngine {
    data_pool: Arc<RwLock<DataPool>>,
    data_writer: Arc<DataWriter>,
    auto_recovery: bool,
}

impl SampleEngine {
    pub fn get_log_path(&self) -> String {
        self.data_writer.log_path.as_ref().unwrap().to_string()
    }

    pub fn recovery_from_file(&self) {
        let f = OpenOptions::new().read(true).open(self.get_log_path());
        match f {
            Err(_) => println!("recovery log not exist!"),
            Ok(f) => {
                let file = BufReader::new(&f);
                let mut it = file.lines();

                let mut line = it.next();
                let mut data_pool = self.data_pool.write().unwrap();
                while line.is_some() {
                    let l = line.unwrap().unwrap();
                    if l == "0" {
                        let key = it.next().unwrap().unwrap();
                        let val = it.next().unwrap().unwrap();
                        data_pool.insert(key.into_bytes(), val.into_bytes());
                    } else if l == "1" {
                        let key = it.next().unwrap().unwrap();
                        data_pool.delete(key.into_bytes());
                    }
                    line = it.next()
                }
            }
        }
    }

    pub fn shutdown(&self) {
        println!("sample engine shutdown ...");
        self.data_writer.record(None);
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
        if self.auto_recovery {
            self.data_writer.record(Some(Task::Put(key.clone(), value.clone())));
        }
        let mut data_pool = self.data_pool.write().unwrap();
        data_pool.insert(key, value);
        Ok(())
    }
    fn delete(&self, key: Key) -> Result<()> {
        if self.auto_recovery {
            self.data_writer.record(Some(Task::Delete(key.clone())));
        }
        let mut data_pool = self.data_pool.write().unwrap();
        data_pool.delete(key);
        Ok(())
    }
}

#[test]
fn engine_test() {
    use std::thread;
    use std::time::Duration;

    let engine = SampleEngineBuilder::new().auto_recovery(false).builder();
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

#[test]
fn auto_recovery_on_test() {
    let engine = SampleEngineBuilder::new().set_log_path("tests/files/foo-test.txt").auto_recovery(true).builder();

    let k = b"k123".to_vec();
    let v = b"v234".to_vec();
    let actual_v = engine.get(k.clone()).ok().unwrap().unwrap();
    assert_eq!(v, actual_v)
}

#[test]
fn auto_recovery_off_test() {
    let engine = SampleEngineBuilder::new().set_log_path("tests/files/foo-test.txt").auto_recovery(false).builder();

    let k = b"k123".to_vec();
    let v = b"v234".to_vec();
    let actual_v = engine.get(k.clone()).ok().unwrap();
    assert_eq!(None, actual_v)
}

#[test]
fn t() {
    let z = "你好";
    let v = z.to_string().into_bytes();
}