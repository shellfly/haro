use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info};
use regex::Regex;

use crate::request::Request;
use crate::response::Response;

type Handler = fn(Request) -> Response;
pub struct Application {
    addr: &'static str,
    routes: Vec<(Regex, Handler)>,
}
impl Application {
    pub fn new(addr: &'static str) -> Self {
        let routes = Vec::new();
        Self { addr, routes }
    }

    pub fn route(&mut self, pattern: &'static str, f: Handler) {
        let p = Regex::new(pattern).unwrap();
        self.routes.push((p, f));
    }

    pub fn run(&self) {
        env_logger::init();

        let listener = TcpListener::bind(self.addr)
            .unwrap_or_else(|error| panic!("Problem starting web server: {:?}", error));
        info!("Started web server on addr {}", self.addr);

        for stream in listener.incoming() {
            let stream = stream.unwrap();

            debug!("Connection established!, {:?}", stream);
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let req = Request::from(&stream);
        info!("{} {}", req.method, req.full_path);
        let handler = self._match(&req.path);
        let res = handler(req);
        let response = format!(
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\n\r\n{}",
            res.status_code, res.status, res.content_length, res.content,
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }

    fn _match(&self, path: &str) -> Handler {
        for (pattern, handler) in &self.routes {
            if pattern.is_match(path) {
                return *handler;
            }
        }
        return notfound;
    }
}

fn notfound(_req: Request) -> Response {
    Response::from("404 Not Found")
}
