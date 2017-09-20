extern crate hyper;
extern crate futures;

use std::ascii::AsciiExt;
use futures::Stream;
use futures::stream::Map;
use futures::future::Future;

use hyper::{Chunk, Body};
use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let servidor = Http::new().bind(&addr, || Ok(DbzServer)).unwrap();
    servidor.run().unwrap();
}

struct DbzServer;

impl Service for DbzServer {

    type Request = Request;
    type Response = Response<Map<Body, fn(Chunk) -> Chunk>>;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Post, "/dbz") => {
                response.set_body(req.body().map(para_maiusculas as _));
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }
}

fn para_maiusculas(chunk: Chunk) -> Chunk {
    let maiusculas = chunk.iter()
        .map(|byte| byte.to_ascii_uppercase())
        .collect::<Vec<u8>>();
    Chunk::from(maiusculas)
}