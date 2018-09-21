extern crate grpcio;
extern crate kvprotos;

use std::env;
use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use kvprotos::src::kvpb::{GetRequest,GetResponse};
use kvprotos::src::kvpb_grpc::KvClient;


fn main() {

    let port_str = "18881";

    let port = port_str
        .parse::<u16>()
        .expect(format!("{} is not a valid port number",port_str ).as_str());

    let env = Arc::new(EnvBuilder::new().build());
    let ch = ChannelBuilder::new(env).connect(format!("localhost:{}", port).as_str());
    let client = KvClient::new(ch);


    let mut request = GetRequest::new();
    request.set_key(b"key-ab".to_vec());
    let ret = client.get(&request).expect("RPC failed");
    println!("Get response = {:?} ",ret )
}
