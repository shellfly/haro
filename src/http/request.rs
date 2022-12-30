use std::collections::HashMap;

use http::Uri;
use log::{debug, error, info};

use crate::http::conn::Conn;
use crate::http::utils::{parse_body, parse_headers, parse_query};

#[derive(Debug)]
pub struct Request {
    pub method: String, //TODO: enum
    pub uri: Uri,
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

        let uri = full_path.parse::<Uri>().unwrap();
        let get = parse_query(uri.query());

        let mut post = HashMap::new();
        if method == "POST" {
            post = parse_body(conn)
        }

        Self {
            method,
            uri,
            version,
            headers,
            get,
            post,
            params: HashMap::new(),
        }
    }

    pub fn path(&self) -> &str {
        self.uri.path()
    }

    pub fn full_path(&self) -> &str {
        if let Some(full_path) = self.uri.path_and_query() {
            return full_path.as_str();
        }
        ""
    }
}
