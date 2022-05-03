use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use log::{debug, error, info};

use crate::utils::{parse_body, parse_headers, parse_query};

#[derive(Debug)]
pub struct Request {
    pub method: String, //TODO: enum
    pub full_path: String,
    pub path: String,
    pub version: String, // TODO: enum
    pub headers: HashMap<String, String>,
    pub get: HashMap<String, String>,
    pub post: HashMap<String, String>,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn from(stream: &TcpStream) -> Self {
        let mut reader = BufReader::new(stream.try_clone().unwrap());
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        let line: Vec<&str> = buf.trim().split(" ").collect();
        let (method, full_path, version) = (
            line[0].to_string(),
            line[1].to_string(),
            line[2].to_string(),
        );
        let headers = parse_headers(&mut reader);
        debug!("{method} {full_path} {version}, headers: {:?}", headers);

        let mut get = HashMap::new();
        let mut post = HashMap::new();

        let path = if let Some(query_start) = full_path.find("?") {
            get = parse_query(&full_path[query_start + 1..]);
            full_path[..query_start].to_string()
        } else {
            full_path.clone()
        };
        if method == "POST" {
            post = parse_body(&mut reader)
        }

        Self {
            method,
            full_path,
            path,
            version,
            headers,
            get,
            post,
            params: HashMap::new(),
        }
    }
}
