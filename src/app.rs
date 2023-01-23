use std::net::TcpStream;
use std::{net::TcpListener, thread};

use log::{debug, info};

use crate::http::conn::Conn;
use crate::http::request::Request;
use crate::pool::ThreadPool;
use crate::router::{Handler, Router};

pub struct Application {
    addr: &'static str,
    router: Router,
}

impl Application {
    pub fn new(addr: &'static str) -> Self {
        env_logger::init();
        let router = Router::default();
        Self { addr, router }
    }

    pub fn route(&mut self, pattern: &'static str, handler: Handler) {
        self.router.add(pattern, handler);
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
            pool.execute(|| {
                handle_connection(router, stream);
            });
        }
    }
}

fn handle_connection(router: Router, stream: TcpStream) {
    let mut conn = Conn::new(stream);
    let mut req = Request::new(&mut conn);
    info!("{} {}", req.method(), req.full_path());
    let (params, handler) = router.dispatch(req.path());
    req.params = params;

    let res = handler(req);

    conn.write_all(res.to_string().as_bytes());
    conn.flush();
}
