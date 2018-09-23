extern crate protobuf;
extern crate grpcio;
extern crate futures;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot::{self, Receiver, Sender};
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink, Server as GrpcServer};

use kvprotos::src::kvpb::{GetRequest, GetResponse, PutRequest, PutResponse, DeleteRequest, DeleteResponse};
use kvprotos::src::kvpb_grpc::{self, Kv};
use storage::Storage;
use storage::engine::sample_engine::SampleEngine;

use storage::engine::Engine;
use std::sync::Mutex;
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
            Err(_) => response.set_error(String::from("errors")),
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
            Err(_) => response.set_error(String::from("errors")),
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
            Err(_) => response.set_error(String::from("errors")),
        }
        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }
}


pub struct KvServer {
    grpc_server: GrpcServer,
    //    chan:Arc<Mutex<(Sender<()>, Receiver<()>)>>
//    chan: (Sender<()>, Receiver<()>),
//
//    rev: Option<&'a Receiver<()>>,
//
//    sender: Option<&'a  Sender<()>>,
}

impl KvServer {
    pub fn new(host: String, port: u16) -> Self {
        let env = Arc::new(Environment::new(1));
        let service = kvpb_grpc::create_kv(KvService::new());
        let grpc_server = ServerBuilder::new(env)
            .register_service(service)
            .bind(host.as_ref(), port.clone()).build().unwrap();


        KvServer {
            grpc_server,
//            chan:Arc::new(Mutex::new(oneshot::channel()))
//            chan: oneshot::channel(),
//            rev: None,
//            sender:None
        }
    }

    pub fn start(&mut self) {
        self.grpc_server.start();

        for &(ref host, port) in self.grpc_server.bind_addrs() {
            println!("listening on {}:{}", host, port);
        }
//        let (tx, rx) = oneshot::channel();
//        thread::spawn(move || {
//            println!("Press ENTER to exit...");
//            let _ = io::stdin().read(&mut [0]).unwrap();
//            tx.send(())
//        });
//        let _ = rx.wait();
//        let _ = self.grpc_server.shutdown().wait();
    }

    pub fn stop(&mut self) {
//        self.sender.unwrap().send(());
        println!("stoping server...");
        self.grpc_server.shutdown();
    }
}


#[test]
fn server_start_test() {
    let host = "127.0.0.1";
    let port = 0;
    let mut server = KvServer::new(host.into(), port);
    server.start();
    server.stop();
}
