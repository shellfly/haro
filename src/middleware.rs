//! Middleware definition and built-in middlewares
//!
//!
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

/// Closure type of handler in order to capture environment variables, required when writing a middleware
pub type DynHandler = Arc<dyn Fn(Request) -> Response>;

/// Function type for a middleware to receive a [`DynHandler`] and return a new [`DynHandler`]
pub type Middleware = fn(next: DynHandler) -> DynHandler;

/// Logging middleware to log every request and response time
/// # Example
/// ```
/// use haro::{Application, middleware}
///
/// let mut app = Application::new("0:8080");
/// app.middleware(middleware::logging);
/// ```
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

/// Change a fn pointer to a closure
/// # Example
/// ```
/// use haro::{Request, Response, middleware};
///
/// fn handler(_:Request) -> Response{
///     Response::str("hello")
/// }
///
/// let dyn_handler = middleware::make_dyn_handler(handler);
/// ```
pub fn make_dyn_handler(h: Handler) -> DynHandler {
    Arc::new(move |req: Request| -> Response { h(req) })
}
