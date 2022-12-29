use std::collections::HashMap;

use log::{debug, error, info};

use crate::conn::Conn;
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
    pub fn new(conn: &mut Conn) -> Self {
        let mut buf = String::new();
        conn.read_line(&mut buf);
        let line: Vec<&str> = buf.trim().split(' ').collect();
        let (method, full_path, version) = (
            line[0].to_string(),
            line[1].to_string(),
            line[2].to_string(),
        );
        let headers = parse_headers(conn);
        debug!("{method} {full_path} {version}, headers: {:?}", headers);

        let mut get = HashMap::new();
        let mut post = HashMap::new();

        let path = if let Some(query_start) = full_path.find('?') {
            get = parse_query(&full_path[query_start + 1..]);
            full_path[..query_start].to_string()
        } else {
            full_path.clone()
        };
        if method == "POST" {
            post = parse_body(conn)
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
