use std::{
    collections::HashMap,
    fmt::Debug,
    io::Write,
    net::{TcpListener, TcpStream},
};

use log::{debug, error, info};
use regex::Regex;

use crate::request::Request;
use crate::response::Response;

type Handler = fn(Request) -> Response;

struct Router {
    routes: Vec<(Rule, Handler)>,
}

impl Router {
    fn new() -> Self {
        Router { routes: Vec::new() }
    }
    fn add(&mut self, pattern: &'static str, f: Handler) {
        let rule = Rule::from(pattern);
        self.routes.push((rule, f));
        self.update_order()
    }
    fn update_order(&mut self) {
        // TODO: compare performance with a radix tree
        self.routes
            .sort_by(|a, b| b.0.num_parts.cmp(&a.0.num_parts));
    }
    fn route(&self, path: &str) -> (HashMap<String, String>, Handler) {
        for (rule, handler) in &self.routes {
            if let Some(params) = rule._match(path) {
                return (params, *handler);
            }
        }
        (HashMap::new(), notfound)
    }
}
#[derive(Debug)]
struct Rule {
    pattern: &'static str,
    num_parts: usize,
    regex: Option<Regex>,
}

impl Rule {
    fn from(pattern: &'static str) -> Self {
        let mut parts = Vec::new();
        let mut has_regex = false;
        for part in pattern.split('/') {
            if let Some(stripped) = part.strip_prefix(':') {
                let regex_part = format!("(?P<{}>.+)", stripped);
                parts.push(regex_part);
                has_regex = true
            } else if !part.is_empty() {
                parts.push(part.to_string())
            }
        }
        let regex = if has_regex {
            let regex_str = format!("/{}", parts.join("/"));
            Some(Regex::new(&regex_str).unwrap())
        } else {
            None
        };
        Rule {
            pattern,
            num_parts: parts.len(),
            regex,
        }
    }

    /// Register a rule for routing incoming requests and building URLs.
    ///
    /// ```
    /// assert!(Rule.from("/")._match("/").is_none())
    /// Rule.from("/abc")._match("/def")
    /// ```
    fn _match(&self, path: &str) -> Option<HashMap<String, String>> {
        if self.regex.is_none() {
            if self.pattern == path {
                return Some(HashMap::new());
            }
        } else {
            let re = self.regex.as_ref().unwrap();
            if let Some(caps) = re.captures(path) {
                // CaptureNames: (Iter([None, Some("aaa")]))
                // captures: Some(Captures({0: Some("aab"), "aaa": Some("aa")}))
                return Some(
                    re.capture_names()
                        .flatten()
                        .filter_map(|n| Some((n.to_string(), caps.name(n)?.as_str().to_string())))
                        .collect(),
                );
            }
        }
        None
    }
}

pub struct Application {
    addr: &'static str,
    router: Router,
}
impl Application {
    pub fn new(addr: &'static str) -> Self {
        env_logger::init();
        let router = Router::new();
        Self { addr, router }
    }

    pub fn route(&mut self, pattern: &'static str, f: Handler) {
        self.router.add(pattern, f);
    }

    pub fn run(&self) {
        let listener = TcpListener::bind(self.addr)
            .unwrap_or_else(|error| panic!("Problem starting web server: {:?}", error));
        info!("Started web server on addr {}", self.addr);
        debug!("routes: \n {:?}", self.router.routes);
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            self.handle_connection(stream);
        }
    }

    fn handle_connection(&self, mut stream: TcpStream) {
        let mut req = Request::from(&stream);
        info!("{} {}", req.method, req.full_path);
        let (params, handler) = self.router.route(&req.path);
        req.params = params;
        let res = handler(req);

        stream.write_all(res.to_string().as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}

fn notfound(_req: Request) -> Response {
    Response::str("404 Not Found")
}
