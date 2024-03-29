use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::thread::available_parallelism;

use log::{debug, info};

use crate::http::conn::Conn;
use crate::middleware::Middleware;
use crate::pool::ThreadPool;
use crate::router::Router;
use crate::{DynHandler, Handler, Request, Response};

/// A web Application with routes and middlewares
pub struct Application {
    addr: &'static str,
    num_threads: usize,
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
        let num_threads = available_parallelism().unwrap_or(default_num_threads).get();
        Self {
            addr,
            num_threads,
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
        self.num_threads = n;
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
    pub fn route<F>(&mut self, pattern: &'static str, f: F)
    where
        F: Fn(Request) -> Response + Send + Sync + 'static,
    {
        self.router.add(pattern, f);
    }

    /// Add a route using a trait type
    /// # Example
    /// ```
    /// use haro::{Application, Request, Response, Handler};
    ///
    /// let mut app = Application::new("0:8080");
    /// let hello_handler = HelloHandler{name:"Haro".to_string()};
    /// app.route_handler("/hello", hello_handler);
    ///
    /// struct HelloHandler {
    ///     name: String,
    /// }
    ///
    /// impl Handler for HelloHandler {
    ///     fn call(&self, _: Request) -> Response {
    ///         Response::str(format!("hello {}", self.name))
    ///     }
    /// }
    pub fn route_handler<H>(&mut self, pattern: &'static str, h: H)
    where
        H: Handler + Send + Sync + 'static,
    {
        self.router.add_handler(pattern, h);
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
        let pool = ThreadPool::new(self.num_threads);

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
