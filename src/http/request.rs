use std::collections::HashMap;

use http::{header::CONTENT_LENGTH, Method, Request as HttpRequest, Version};

use crate::http::{
    conn::Conn,
    utils::{parse_json_body, parse_query, read_headers},
};

#[derive(Debug)]
pub struct Request {
    req: HttpRequest<Vec<u8>>,
    pub args: HashMap<String, String>,
    pub data: HashMap<String, String>,
    pub params: HashMap<String, String>,
}

impl Request {
    pub fn new(conn: &mut Conn) -> Self {
        // parse method, uri and version
        let mut buf = String::new();
        conn.read_line(&mut buf);
        let line: Vec<&str> = buf.trim().split(' ').collect();
        let (method, uri, version) = (line[0], line[1], line[2]);
        let version = match version {
            "HTTP/2.0" => Version::HTTP_2,
            "HTTP/3.0" => Version::HTTP_3,
            _ => Version::HTTP_11,
        };
        let mut builder = HttpRequest::builder()
            .method(method)
            .uri(uri)
            .version(version);

        // parse headers
        let mut content_length = 0;
        for (key, value) in read_headers(conn) {
            if key.to_lowercase() == CONTENT_LENGTH.as_str().to_lowercase() {
                content_length = value.parse().unwrap();
            }
            builder = builder.header(key, value);
        }

        // parse body
        let mut body = Vec::<u8>::new();
        body.resize(content_length, 0);
        conn.read_exact(&mut body);
        let req = builder.body(body).unwrap();

        let args = parse_query(req.uri().query());
        let mut data = HashMap::new();
        if method == Method::POST && content_length > 0 {
            data = parse_json_body(req.body())
        }

        Self {
            req,
            args,
            data,
            params: HashMap::new(),
        }
    }

    pub fn method(&self) -> &str {
        self.req.method().as_str()
    }

    pub fn path(&self) -> &str {
        self.req.uri().path()
    }

    pub fn full_path(&self) -> &str {
        if let Some(full_path) = self.req.uri().path_and_query() {
            return full_path.as_str();
        }
        ""
    }
}