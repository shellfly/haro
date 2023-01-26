use std::collections::HashMap;
use std::net::TcpStream;
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::{net::TcpListener, thread};

use log::{debug, info};

use crate::http::conn::Conn;
use crate::middleware::{make_dyn_handler, Middleware};
use crate::pool::ThreadPool;
use crate::router::{Handler, Router};
use crate::{Request, Response};

/// A web Application with routes and middlewares
pub struct Application {
    addr: &'static str,
    num_threads: NonZeroUsize,
    router: Router,
    middlewares: Vec<Arc<Middleware>>,
}

impl Application {
    /// Create a new `Application` instance
    /// # Examples
    /// ```
    /// use web::Application;
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
    /// use web::Application;
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
    /// use web::{Application, middleware};
    ///
    /// let mut app = Application::new("0:8080");
    /// app.middleware(middleware::logging);
    /// ```
    pub fn middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(Arc::new(middleware));
    }

    /// Add a route into an `Application`
    /// # Example
    /// ```
    /// use web::{Application, Request, Response, middleware};
    ///
    /// let mut app = Application::new("0:8080");
    /// app.route("/", index);
    ///
    /// fn index(_:Request) -> Response {
    ///     Response::str("hello web.rs")
    /// }
    /// ```
    pub fn route(&mut self, pattern: &'static str, handler: Handler) {
        self.router.add(pattern, handler);
    }

    /// Send a request to an `Application`, usually used in test
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use web::{Application, Request, Response};
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
        let (params, handler) = self.router.dispatch(req.path());
        req.params = params;

        // TODO: how much benefits to move applying middlewares at begging to avoid do it every time in a new request.
        let mut dyn_handler = make_dyn_handler(handler);
        // apply middleware in reverse order
        for middleware in self.middlewares.iter().rev() {
            dyn_handler = middleware(dyn_handler);
        }
        dyn_handler(req)
    }

    /// Run the application, start listening on the specify address and start a worker pool to handle requests
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use web::{Application, Request, Response};
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

fn handle_connection(router: Router, middlewares: Vec<Arc<Middleware>>, stream: TcpStream) {
    let mut conn = Conn::from(stream);
    let mut req = Request::from(&mut conn);
    let (params, handler) = router.dispatch(req.path());
    req.params = params;

    let mut dyn_handler = make_dyn_handler(handler);
    // apply middleware in reverse order
    for middleware in middlewares.iter().rev() {
        dyn_handler = middleware(dyn_handler);
    }
    let res = dyn_handler(req);

    conn.write_all(res.to_string().as_bytes());
    conn.flush();
}
