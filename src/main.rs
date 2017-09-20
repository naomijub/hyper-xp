extern crate hyper;
extern crate futures;

use futures::future::Future;

use hyper::header::ContentLength;
use hyper::server::{Http, Request, Response, Service};

fn main() {
    let addr = "127.0.0.1:3000".parse().unwrap();
    let servidor = Http::new().bind(&addr, || Ok(Ola)).unwrap();
    servidor.run().unwrap();
}

struct Ola;

const FRASE: &'static str = "Ola Goku!";

impl Service for Ola {

    type Request = Request;
    type Response = Response;
    type Error = hyper::Error;
    type Future = Box<Future<Item=Self::Response, Error=Self::Error>>;

    fn call(&self, _req: Request) -> Self::Future {
        Box::new(futures::future::ok(
            Response::new()
                .with_header(ContentLength(FRASE.len() as u64))
                .with_body(FRASE)
        ))
    }
}