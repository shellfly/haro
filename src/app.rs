use std::{net::TcpListener, thread};

use log::{debug, info};

use crate::conn::Conn;
use crate::request::Request;
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

    pub fn route(&mut self, pattern: &'static str, f: Handler) {
        self.router.add(pattern, f);
    }

    pub fn run(&self) {
        info!("Started web server on addr {}", self.addr);
        debug!("routes: \n {:}", self.router);
        let listener = TcpListener::bind(self.addr)
            .unwrap_or_else(|error| panic!("Problem starting web server: {:?}", error));

        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let conn = Conn::new(stream);
            // TODO: anyway to avoid clone?
            let router = self.router.clone();
            thread::spawn(|| handle_connection(router, conn));
        }
    }
    // pub fn run(&self) {
    //     let rt = tokio::runtime::Runtime::new().unwrap();
    //     rt.block_on(self.run_async())
    // }

    // async fn run_async(&self) {
    // info!("Started web server on addr {}", self.addr);
    // debug!("routes: \n {:}", self.router);
    // let addr_split = self.addr.split(':').collect::<Vec<&str>>();
    // assert!(addr_split.len() == 2);
    // let (host, port) = (addr_split[0], addr_split[1]);

    // let listener = TcpListener::bind(self.addr)
    //     .await
    //     .unwrap_or_else(|error| panic!("Problem starting web server: {:?}", error));

    // loop {
    //     let (stream, _) = listener.accept().await.unwrap();
    //     // TODO: anyway to avoid clone?
    //     // tokio::spawn needs static lifetime, so we can't access self in spawn
    //     let router = self.router.clone();
    //     tokio::spawn(async move {
    //         let mut conn = Conn::new(stream);
    //         handle_connection(router, &mut conn).await;
    //     });
    // }
    //}
}

fn handle_connection(router: Router, mut conn: Conn) {
    let mut req = Request::new(&mut conn);
    info!("{} {}", req.method, req.full_path);
    let (params, handler) = router.dispatch(&req.path);
    req.params = params;
    let res = handler(req);

    conn.write_all(res.to_string().as_bytes());
    conn.flush();
}
