extern crate protobuf;
extern crate grpcio;
extern crate futures;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use kvprotos::src::kvpb::{GetRequest, GetResponse, PutRequest, PutResponse, DeleteRequest, DeleteResponse};
use kvprotos::src::kvpb_grpc::{self, Kv};
use storage::Storage;
use storage::engine::sample_engine::SampleEngine;

use storage::engine::Engine;

//
//struct MemoryDB{
//
//}
//
//impl Engine for MemoryDB{
//    fn get(&self, key: &Key) -> Result<Option<Value>>{
//        unimplemented!()
//    }
//
//    fn put(&self,  key: Key, value: Value) -> Result<()> {
//        unimplemented!()
//    }
//
//    fn delete(&self,key:Key)->Result<()>{
//        unimplemented!()
//    }
//}


#[derive(Clone)]
struct KvService {
    storage: Storage<SampleEngine>,
}

impl KvService {
    pub fn new() -> Self {
        KvService {
            storage: Storage::new()
        }
    }
}

impl Kv for KvService {
    fn get(&self, ctx: RpcContext, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut response = GetResponse::new();
        println!("Received GetRequest {{ {:?} }}", req);
        let engine = &self.storage.engine;
        let ret = engine.get(req.key);
        match ret {
            Ok(op) => match op {
                Some(key) => response.set_value(key),
                None => (),
            }
           Err(_)=> response.set_error(String::from("errors")),
        }

        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }

    fn put(&self, ctx: RpcContext, req: PutRequest, sink: UnarySink<PutResponse>) {
        let mut response = PutResponse::new();
        println!("Received PutRequest {{ {:?} }}", req);
        let engine = &self.storage.engine;
        let ret = engine.put(req.key, req.value);

        match ret {
            Ok(_) => (),
           Err(_)=> response.set_error(String::from("errors")),
        }

        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }

    fn delete(&self, ctx: RpcContext, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {
        let mut response = DeleteResponse::new();
        println!("Received DeleteResponse {{ {:?} }}", req);
        let engine = &self.storage.engine;
        let ret = engine.delete(req.key);
        match ret {
            Ok(_) => (),
           Err(_)=> response.set_error(String::from("errors")),
        }
        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }
}


pub struct KvServer {
    host: String,
    port: u16,
}

impl KvServer {
    pub fn new(host: String, port: u16) -> Self {
        KvServer {
            host,
            port,
        }
    }

    pub fn start(&self) {
        let env = Arc::new(Environment::new(1));
        let service = kvpb_grpc::create_kv(KvService::new());
        let mut server = ServerBuilder::new(env)
            .register_service(service)
            .bind(self.host.as_ref(), self.port.clone())
            .build()
            .unwrap();

        server.start();

        for &(ref host, port) in server.bind_addrs() {
            println!("listening on {}:{}", host, port);
        }
        let (tx, rx) = oneshot::channel();
        thread::spawn(move || {
            println!("Press ENTER to exit...");
//            let _ = io::stdin().read(&mut [0]).unwrap();
//            tx.send(())
        });
        let _ = rx.wait();
        let _ = server.shutdown().wait();
    }
}


#[test]
fn server_start_test() {
    let host ="127.0.0.1";
    let port = 0;
    let server = KvServer::new(host.into(),port);
    server.start();
}
