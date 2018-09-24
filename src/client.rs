extern crate grpcio;

use std::sync::Arc;

use grpcio::{ChannelBuilder, EnvBuilder};

use kvprotos::src::kvpb::{GetRequest, PutRequest, DeleteRequest};
use kvprotos::src::kvpb_grpc::KvClient;
use storage::{Key, Value};
use server::KvServer;


struct Client {
    client: KvClient,
}

impl Client {
    pub fn new(host: String, port: u16) -> Self {
        let addr = format!("{}:{}", host, port);
        let env = Arc::new(EnvBuilder::new().build());
        let ch = ChannelBuilder::new(env).connect(addr.as_ref());
        let kv_client = KvClient::new(ch);

        Client {
            client: kv_client,
        }
    }
    pub fn get(&self, k: Key) -> String {
        let mut request = GetRequest::new();
        request.set_key(k);
        let ret = self.client.get(&request).expect("RPC failed");
        unsafe { String::from_utf8_unchecked(ret.value) }
    }

    pub fn put(&self, k: Key, v: Value) {
        let mut put_req = PutRequest::new();
        put_req.set_key(k);
        put_req.set_value(v);
        self.client.put(&put_req).expect("RPC failed");
    }

    pub fn delete(&self, k: Key) {
        let mut del_req = DeleteRequest::new();
        del_req.set_key(k);
        self.client.delete(&del_req).expect("RPC failed");
    }
}


#[test]
fn client_test() {
    let test_key = b"key-test".to_vec();
    let test_value = b"value-test".to_vec();

    let test_host = String::from("127.0.0.1");
    let test_port = 18811;

    let client = Client::new(test_host.clone(), test_port);
    let mut test_server = KvServer::new(test_host.clone(), test_port);

    test_server.start();
    client.delete(test_key.clone());

    let v = client.get(test_key.clone());
    assert_eq!("", v);

    client.put(test_key.clone(), test_value.clone());
    let v = client.get(test_key.clone());
    assert_eq!(String::from_utf8(test_value.clone()).unwrap(), v);

    client.delete(test_key.clone());
    let v = client.get(test_key.clone());
    assert_eq!("", v);

    let test_key = "你好".to_string().into_bytes();
    let test_value = "世界".to_string().into_bytes();
    client.put(test_key.clone(), test_value.clone());
    let v = client.get(test_key.clone());
    assert_eq!("世界", v);


    use std::thread;
    use std::time::Duration;

    thread::sleep(Duration::from_millis(100));

    test_server.stop();
}