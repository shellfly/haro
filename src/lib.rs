use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info};

pub struct Application<'a> {
    addr: &'a str,
    routes: HashMap<&'a str, fn() -> String>,
}
impl<'a> Application<'a> {
    pub fn new(addr: &'a str) -> Self {
        let routes = HashMap::new();
        Self { addr, routes }
    }

    pub fn route(&mut self, url: &'a str, f: fn() -> String) {
        self.routes.insert(url, f);
    }

    pub fn run(&self) {
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
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        let line: Vec<&str> = buf.trim().split(" ").collect();
        let (method, path, version) = (line[0], line[1], line[2]);
        info!("{method} {path} {version}");

        let headers = read_headers(reader);
        debug!("headers: {:?}", headers);

        let handler = self.routes.get(path).unwrap();
        let contents = handler();
        let response = format!(
            "HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n{}",
            contents.len(),
            contents
        );
        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn read_headers(mut reader: BufReader<TcpStream>) -> Vec<String> {
    let mut headers = Vec::new();
    loop {
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        if buf == "\r\n" {
            break;
        }
        headers.push(buf.trim().to_string());
    }
    headers
}
