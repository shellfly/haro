use std::collections::HashMap;

use cookie::Cookie;
use http::{
    header::{CONTENT_LENGTH, CONTENT_TYPE, COOKIE},
    HeaderMap, HeaderValue, Request as HttpRequest, Version,
};
use log::warn;

use crate::http::{
    conn::Conn,
    utils::{parse_json_body, parse_query, read_headers},
};

/// HTTP Request
#[derive(Debug)]
pub struct Request {
    req: HttpRequest<Vec<u8>>,
    pub args: HashMap<String, String>,
    pub data: HashMap<String, String>,
    pub params: HashMap<String, String>,
}

impl Request {
    /// Create a new `Request`
    /// # Example
    /// ```
    /// use std::collections::HashMap;
    /// use haro::Request;
    ///
    /// let headers = HashMap::new();
    /// let body = &Vec::new();
    /// let mut req = Request::new("get", "/", headers, body);
    /// ```
    pub fn new(method: &str, uri: &str, headers: HashMap<String, String>, body: &[u8]) -> Self {
        let mut builder = HttpRequest::builder().method(method).uri(uri);
        let content_length = body.len();
        let mut content_type = String::new();
        for (key, value) in headers {
            if key.to_lowercase() == CONTENT_TYPE.as_str().to_lowercase() {
                content_type = value.clone();
            }
            builder = builder.header(key, value);
        }

        let req = builder
            .header(CONTENT_LENGTH, body.len())
            .body(body.to_vec())
            .unwrap();

        let args = parse_query(req.uri().query());
        let mut data = HashMap::new();
        if content_length > 0 {
            data = match content_type.as_str() {
                "application/json" => parse_json_body(req.body()),
                _ => {
                    warn!("unsupported content type {}", content_type);
                    HashMap::new()
                }
            };
        }
        Self {
            req,
            args,
            data,
            params: HashMap::new(),
        }
    }
    /// Create a new `Request` from a TCP connectio
    pub fn from(conn: &mut Conn) -> Self {
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
        let mut content_type = String::new();
        for (key, value) in read_headers(conn) {
            if key.to_lowercase() == CONTENT_LENGTH.as_str().to_lowercase() {
                content_length = value.parse().unwrap();
            } else if key.to_lowercase() == CONTENT_TYPE.as_str().to_lowercase() {
                content_type = value.clone();
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
        if content_length > 0 {
            data = match content_type.as_str() {
                "application/json" => parse_json_body(req.body()),
                _ => {
                    // TODO: support more content types
                    warn!("unsupported content type {}", content_type);
                    HashMap::new()
                }
            };
        }

        Self {
            req,
            args,
            data,
            params: HashMap::new(),
        }
    }

    /// HTTP method for current `Request`
    pub fn method(&self) -> &str {
        self.req.method().as_str()
    }

    /// HTTP path for current `Request`
    pub fn path(&self) -> &str {
        self.req.uri().path()
    }

    /// HTTP full path with query args
    pub fn full_path(&self) -> &str {
        if let Some(full_path) = self.req.uri().path_and_query() {
            return full_path.as_str();
        }
        ""
    }

    /// HTTP headers for current `Request`
    pub fn headers(&self) -> &HeaderMap<HeaderValue> {
        self.req.headers()
    }

    /// HTTP cookies for current `Request`
    pub fn cookies(&self) -> HashMap<String, String> {
        let headers = self.headers();
        let cookies = headers
            .get(COOKIE)
            .map(|v| Cookie::split_parse(v.to_str().unwrap()));

        let mut cookies_map = HashMap::new();
        for cookie in cookies.into_iter().flatten().flatten() {
            cookies_map.insert(cookie.name().to_string(), cookie.value().to_string());
        }

        cookies_map
    }
}
