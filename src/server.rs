extern crate futures;
extern crate grpcio;
extern crate kvprotos;

use std::io::Read;
use std::sync::Arc;
use std::{io, thread};

use futures::sync::oneshot;
use futures::Future;
use grpcio::{Environment, RpcContext, ServerBuilder, UnarySink};

use kvprotos::src::kvpb::{ GetRequest, GetResponse, PutRequest, PutResponse, DeleteRequest, DeleteResponse};
use kvprotos::src::kvpb_grpc::{self, Kv};

#[derive(Clone)]
struct KvService;

impl Kv for KvService {

    fn get(&self, ctx: RpcContext, req: GetRequest, sink: UnarySink<GetResponse>) {
        let mut response = GetResponse::new();
        println!("Received GetRequest {{ {:?} }}", req);
        response.set_value(b"value-cd".to_vec());

        let f = sink.success(response.clone())
            .map(move |_| println!("Responded with  {{ {:?} }}", response))
            .map_err(move |err| eprintln!("Failed to reply: {:?}", err));
        ctx.spawn(f)
    }
    fn put(&self, ctx: RpcContext, req: PutRequest, sink: UnarySink<PutResponse>) {}
    fn delete(&self, ctx: RpcContext, req: DeleteRequest, sink: UnarySink<DeleteResponse>) {}
}

fn main() {
    let env = Arc::new(Environment::new(1));
    let service = kvpb_grpc::create_kv(KvService);
    let mut server = ServerBuilder::new(env)
        .register_service(service)
        .bind("127.0.0.1", 18881)
        .build()
        .unwrap();
    server.start();
    for &(ref host, port) in server.bind_addrs() {
        println!("listening on {}:{}", host, port);
    }
    let (tx, rx) = oneshot::channel();
    thread::spawn(move || {
        println!("Press ENTER to exit...");
        let _ = io::stdin().read(&mut [0]).unwrap();
        tx.send(())
    });
    let _ = rx.wait();
    let _ = server.shutdown().wait();
}
