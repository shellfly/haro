//! Middleware definition and built-in middlewares
//!
//!
use std::sync::Arc;
use std::time::Instant;

use log::info;

use crate::{DynHandler, Request, Response};

/// Arc of trait object for Middleware type to receive a [`DynHandler`] and return a new [`DynHandler`]
pub type Middleware = Arc<dyn Fn(DynHandler) -> DynHandler + Send + Sync>;

/// Logging middleware to log every request and response time
/// # Example
/// ```
/// use haro::{Application, middleware};
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
