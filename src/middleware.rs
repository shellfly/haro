use std::sync::Arc;
use std::time::Instant;

use log::info;

use crate::{router::Handler, Request, Response};

// pub trait Middleware {
//     fn process_request(&self, _: &mut Request) -> Option<Response> {
//         None
//     }
//     fn process_response(&self, _: &mut Response) {}
// }

// pub struct Logging {}

// impl Middleware for Logging {
//     fn process_request(&self, req: &mut Request) -> Option<Response> {
//         info!("{} {}", req.method(), req.full_path());
//         None
//     }
// }

// use DynHandler closure to capture `req`
pub type DynHandler = Arc<dyn Fn(Request) -> Response>;
pub type Middleware = fn(next: DynHandler) -> DynHandler;

pub fn logging(next: DynHandler) -> DynHandler {
    Arc::new(move |req: Request| -> Response {
        let (method, path) = (req.method().to_string(), req.full_path().to_string());
        info!("{} {}", method, path);
        let start = Instant::now();

        let res = next(req);

        let duration = start.elapsed();
        info!("{} {} finished in {:?}", method, path, duration);

        res
    })
}

pub fn make_dyn_handler(h: Handler) -> DynHandler {
    Arc::new(move |req: Request| -> Response { h(req) })
}
