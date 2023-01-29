use std::collections::HashMap;
use std::net::TcpStream;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::{net::TcpListener, thread};

use log::{debug, info};

use crate::http::conn::Conn;
use crate::middleware::Middleware;
use crate::pool::ThreadPool;
use crate::router::Router;
use crate::{DynHandler, Request, Response};

/// A web Application with routes and middlewares
pub struct Application {
    addr: &'static str,
    num_threads: NonZeroUsize,
    router: Router,
    middlewares: Vec<Middleware>,
}

impl Application {
    /// Create a new `Application` instance
    /// # Examples
    /// ```
    /// use haro::Application;
    ///
    /// let mut app = Application::new("0:12345");
    /// ```
    pub fn new(addr: &'static str) -> Self {
        env_logger::init();
        let router = Router::default();
        let middlewares = Vec::new();
        let default_num_threads = NonZeroUsize::new(8).unwrap();
        Self {
            addr,
            num_threads: default_num_threads,
            router,
            middlewares,
        }
    }

    /// Set thread worker pool size for `Application`
    /// # Examples
    /// ```
    /// use haro::Application;
    ///
    /// let mut app = Application::new("0:8080").num_threads(64);
    /// ```
    pub fn num_threads(mut self, n: usize) -> Self {
        self.num_threads = NonZeroUsize::new(n).unwrap();
        self
    }

    /// Add a middleware into an `Application`
    /// # Example
    /// ```
    /// use haro::{Application, middleware};
    ///
    /// let mut app = Application::new("0:8080");
    /// app.middleware(middleware::logging);
    /// ```
    pub fn middleware<M>(&mut self, middleware: M)
    where
        M: Fn(DynHandler) -> DynHandler + Send + Sync + 'static,
    {
        self.middlewares.push(Arc::new(middleware));
    }

    /// Add a route using a function or closure
    /// # Example
    /// ```
    /// use haro::{Application, Request, Response, middleware};
    ///
    /// let mut app = Application::new("0:8080");
    /// app.route("/", |_| Response::str("Hello Haro"));
    /// app.route("/hello", hello);
    ///
    /// fn hello(_:Request) -> Response {
    ///     Response::str("Hello Haro")
    /// }
    /// ```
    pub fn route<F>(&mut self, pattern: &'static str, handler: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.router.add(pattern, handler);
    }

    /// Send a request to an `Application`, usually used in test
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use haro::{Application, Request, Response};
    ///
    /// fn test_handler(_:Request) -> Response {
    ///     Response::str("test")
    /// }
    ///
    /// let mut app = Application::new("0:12345");
    /// app.route("/", test_handler);
    ///
    /// let res = app.request("get", "/", HashMap::new(), &Vec::new());
    /// assert_eq!("test".as_bytes(), res.body());
    /// ```
    pub fn request(
        &self,
        method: &str,
        uri: &str,
        headers: HashMap<String, String>,
        body: &[u8],
    ) -> Response {
        let mut req = Request::new(method, uri, headers, body);
        let (params, mut handler) = self.router.dispatch(req.path());
        req.params = params;

        // TODO: how much benefits to move applying middlewares at begging to avoid do it every time in a new request.
        // apply middleware in reverse order
        for middleware in self.middlewares.iter().rev() {
            handler = middleware(handler);
        }
        handler(req)
    }

    /// Run the application, start listening on the specify address and start a worker pool to handle requests
    /// # Examples
    /// ```no_run
    /// use std::collections::HashMap;
    /// use haro::{Application, Request, Response};
    ///
    /// fn test_handler(_:Request) -> Response {
    ///     Response::str("test")
    /// }
    ///
    /// let mut app = Application::new("0:12345");
    /// app.run()
    /// ```
    pub fn run(&self) {
        info!("Started web server on addr {}", self.addr);
        debug!("routes: \n {:}", self.router);
        // TODO: should pool size larger than CPU cores since many can be waiting for I/O
        let default_pool_size = NonZeroUsize::new(8).unwrap();
        let size = thread::available_parallelism()
            .unwrap_or(default_pool_size)
            .get();
        let pool = ThreadPool::new(size);

        let listener = TcpListener::bind(self.addr).unwrap();
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            // TODO: anyway to avoid clone?
            let router = self.router.clone();
            let middlewares = self.middlewares.clone();
            pool.execute(|| {
                handle_connection(router, middlewares, stream);
            });
        }
    }
}

fn handle_connection(router: Router, middlewares: Vec<Middleware>, stream: TcpStream) {
    let mut conn = Conn::from(stream);
    let mut req = Request::from(&mut conn);
    let (params, mut handler) = router.dispatch(req.path());
    req.params = params;

    // apply middleware in reverse order
    for middleware in middlewares.iter().rev() {
        handler = middleware(handler);
    }
    let res = handler(req);

    conn.write_all(res.to_string().as_bytes());
    conn.flush();
}

#[cfg(test)]
mod tests {
    use super::Application;

    #[test]
    fn it_works() {
        Application::new("0:65530").num_threads(2);
    }
}
