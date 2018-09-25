// This file is generated. Do not edit
// @generated

// https://github.com/Manishearth/rust-clippy/issues/702
#![allow(unknown_lints)]
#![allow(clippy)]

#![cfg_attr(rustfmt, rustfmt_skip)]

#![allow(box_pointers)]
#![allow(dead_code)]
#![allow(missing_docs)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]
#![allow(trivial_casts)]
#![allow(unsafe_code)]
#![allow(unused_imports)]
#![allow(unused_results)]

const METHOD_KV_GET: ::grpcio::Method<super::kvpb::GetRequest, super::kvpb::GetResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kvpb.Kv/Get",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_PUT: ::grpcio::Method<super::kvpb::PutRequest, super::kvpb::PutResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kvpb.Kv/Put",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_DELETE: ::grpcio::Method<super::kvpb::DeleteRequest, super::kvpb::DeleteResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kvpb.Kv/Delete",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

const METHOD_KV_FIND_NEXT: ::grpcio::Method<super::kvpb::FindNextRequest, super::kvpb::FindNextResponse> = ::grpcio::Method {
    ty: ::grpcio::MethodType::Unary,
    name: "/kvpb.Kv/FindNext",
    req_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
    resp_mar: ::grpcio::Marshaller { ser: ::grpcio::pb_ser, de: ::grpcio::pb_de },
};

pub struct KvClient {
    client: ::grpcio::Client,
}

impl KvClient {
    pub fn new(channel: ::grpcio::Channel) -> Self {
        KvClient {
            client: ::grpcio::Client::new(channel),
        }
    }

    pub fn get_opt(&self, req: &super::kvpb::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvpb::GetResponse> {
        self.client.unary_call(&METHOD_KV_GET, req, opt)
    }

    pub fn get(&self, req: &super::kvpb::GetRequest) -> ::grpcio::Result<super::kvpb::GetResponse> {
        self.get_opt(req, ::grpcio::CallOption::default())
    }

    pub fn get_async_opt(&self, req: &super::kvpb::GetRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::GetResponse>> {
        self.client.unary_call_async(&METHOD_KV_GET, req, opt)
    }

    pub fn get_async(&self, req: &super::kvpb::GetRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::GetResponse>> {
        self.get_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_opt(&self, req: &super::kvpb::PutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvpb::PutResponse> {
        self.client.unary_call(&METHOD_KV_PUT, req, opt)
    }

    pub fn put(&self, req: &super::kvpb::PutRequest) -> ::grpcio::Result<super::kvpb::PutResponse> {
        self.put_opt(req, ::grpcio::CallOption::default())
    }

    pub fn put_async_opt(&self, req: &super::kvpb::PutRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::PutResponse>> {
        self.client.unary_call_async(&METHOD_KV_PUT, req, opt)
    }

    pub fn put_async(&self, req: &super::kvpb::PutRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::PutResponse>> {
        self.put_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_opt(&self, req: &super::kvpb::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvpb::DeleteResponse> {
        self.client.unary_call(&METHOD_KV_DELETE, req, opt)
    }

    pub fn delete(&self, req: &super::kvpb::DeleteRequest) -> ::grpcio::Result<super::kvpb::DeleteResponse> {
        self.delete_opt(req, ::grpcio::CallOption::default())
    }

    pub fn delete_async_opt(&self, req: &super::kvpb::DeleteRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::DeleteResponse>> {
        self.client.unary_call_async(&METHOD_KV_DELETE, req, opt)
    }

    pub fn delete_async(&self, req: &super::kvpb::DeleteRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::DeleteResponse>> {
        self.delete_async_opt(req, ::grpcio::CallOption::default())
    }

    pub fn find_next_opt(&self, req: &super::kvpb::FindNextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<super::kvpb::FindNextResponse> {
        self.client.unary_call(&METHOD_KV_FIND_NEXT, req, opt)
    }

    pub fn find_next(&self, req: &super::kvpb::FindNextRequest) -> ::grpcio::Result<super::kvpb::FindNextResponse> {
        self.find_next_opt(req, ::grpcio::CallOption::default())
    }

    pub fn find_next_async_opt(&self, req: &super::kvpb::FindNextRequest, opt: ::grpcio::CallOption) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::FindNextResponse>> {
        self.client.unary_call_async(&METHOD_KV_FIND_NEXT, req, opt)
    }

    pub fn find_next_async(&self, req: &super::kvpb::FindNextRequest) -> ::grpcio::Result<::grpcio::ClientUnaryReceiver<super::kvpb::FindNextResponse>> {
        self.find_next_async_opt(req, ::grpcio::CallOption::default())
    }
    pub fn spawn<F>(&self, f: F) where F: ::futures::Future<Item = (), Error = ()> + Send + 'static {
        self.client.spawn(f)
    }
}

pub trait Kv {
    fn get(&self, ctx: ::grpcio::RpcContext, req: super::kvpb::GetRequest, sink: ::grpcio::UnarySink<super::kvpb::GetResponse>);
    fn put(&self, ctx: ::grpcio::RpcContext, req: super::kvpb::PutRequest, sink: ::grpcio::UnarySink<super::kvpb::PutResponse>);
    fn delete(&self, ctx: ::grpcio::RpcContext, req: super::kvpb::DeleteRequest, sink: ::grpcio::UnarySink<super::kvpb::DeleteResponse>);
    fn find_next(&self, ctx: ::grpcio::RpcContext, req: super::kvpb::FindNextRequest, sink: ::grpcio::UnarySink<super::kvpb::FindNextResponse>);
}

pub fn create_kv<S: Kv + Send + Clone + 'static>(s: S) -> ::grpcio::Service {
    let mut builder = ::grpcio::ServiceBuilder::new();
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_GET, move |ctx, req, resp| {
        instance.get(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_PUT, move |ctx, req, resp| {
        instance.put(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_DELETE, move |ctx, req, resp| {
        instance.delete(ctx, req, resp)
    });
    let instance = s.clone();
    builder = builder.add_unary_handler(&METHOD_KV_FIND_NEXT, move |ctx, req, resp| {
        instance.find_next(ctx, req, resp)
    });
    builder.build()
}
