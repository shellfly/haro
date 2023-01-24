use std::collections::HashMap;
use std::net::TcpStream;
use std::sync::Arc;
use std::{net::TcpListener, thread};

use log::{debug, info};

use crate::http::conn::Conn;
use crate::middleware::{make_dyn_handler, Middleware};
use crate::pool::ThreadPool;
use crate::router::{Handler, Router};
use crate::{Request, Response};

pub struct Application {
    addr: &'static str,
    router: Router,
    middlewares: Vec<Arc<Middleware>>,
}

impl Application {
    /// ```
    /// use std::collections::HashMap;
    /// use web::{Application, Request, Response};
    /// fn test_handler(_:Request) -> Response {
    ///     Response::str("test")
    /// }
    /// let mut app = Application::new("0:12345");
    /// app.route("/", test_handler);
    /// let res = app.request("get", "/", HashMap::new(), &Vec::new());
    /// assert_eq!("test".as_bytes(), res.body());
    /// ```
    pub fn new(addr: &'static str) -> Self {
        env_logger::init();
        let router = Router::default();
        let middlewares = Vec::new();
        Self {
            addr,
            router,
            middlewares,
        }
    }

    pub fn middleware(&mut self, middleware: Middleware) {
        self.middlewares.push(Arc::new(middleware));
    }

    pub fn route(&mut self, pattern: &'static str, handler: Handler) {
        self.router.add(pattern, handler);
    }

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

        let mut dyn_handler = make_dyn_handler(handler);
        // apply middleware in reverse order
        for middleware in self.middlewares.iter().rev() {
            dyn_handler = middleware(dyn_handler);
        }
        dyn_handler(req)
    }

    pub fn run(&self) {
        info!("Started web server on addr {}", self.addr);
        debug!("routes: \n {:}", self.router);
        let size = thread::available_parallelism().unwrap().get();
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
    let mut conn = Conn::new(stream);
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
