pub mod engine;

use self::engine::Engine;
use self::engine::sample_engine::{SampleEngine, SampleEngineBuilder};


pub type Key = Vec<u8>;
pub type Value = Vec<u8>;

#[derive(Clone)]
pub struct Storage<E: Engine> {
    pub engine: E,
    pub name: String,
}

impl Storage<SampleEngine> {
    pub fn new() -> Self {
        Storage {
            engine: SampleEngineBuilder::new().build(),
            name: "Sample_Storage".to_string(),
        }
    }

    pub fn stop(&self) {
        self.engine.shutdown();
    }
}
