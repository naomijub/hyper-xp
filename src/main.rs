extern crate hyper;
extern crate futures;

use futures::future::Future;

use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let servidor = Http::new().bind(&addr, || Ok(DbzServer)).unwrap();
    servidor.run().unwrap();
}

struct DbzServer;

const FRASE: &'static str = "Ola Goku!";

impl Service for DbzServer {

    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        let mut response = Response::new();

        match (req.method(), req.path()) {
            (&Method::Get, "/") => {
                response.set_body(FRASE);
            },
            (&Method::Post, "/dbz") => {
                response.set_body(req.body());
            },
            _ => {
                response.set_status(StatusCode::NotFound);
            },
        };

        Box::new(futures::future::ok(response))
    }
}