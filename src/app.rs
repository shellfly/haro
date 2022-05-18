use std::{
    collections::HashMap,
    fmt::{Debug, Display},
};

use log::{debug, error, info};
use regex::Regex;
use tokio::net::TcpListener;

use crate::conn::Conn;
use crate::request::Request;
use crate::response::Response;
//use crate::utils::get_function_name;

type Handler = fn(Request) -> Response;

#[derive(Clone)]
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
    fn dispatch(&self, path: &str) -> (HashMap<String, String>, Handler) {
        for (rule, handler) in &self.routes {
            if let Some(params) = rule._match(path) {
                return (params, *handler);
            }
        }
        (HashMap::new(), notfound)
    }
}

impl Display for Router {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let v: Vec<String> = self.routes.iter().map(|r| r.0.to_string()).collect();
        write!(f, "{}", v.join("\n"))
    }
}

#[derive(Debug, Clone)]
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
impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pattern)
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
        let rt = tokio::runtime::Runtime::new().unwrap();
        rt.block_on(self.run_async())
    }

    async fn run_async(&self) {
        let listener = TcpListener::bind(self.addr)
            .await
            .unwrap_or_else(|error| panic!("Problem starting web server: {:?}", error));
        info!("Started web server on addr {}", self.addr);
        debug!("routes: \n {:}", self.router);
        loop {
            let (stream, _) = listener.accept().await.unwrap();
            // TODO: anyway to avoid clone?
            // tokio::spawn needs static lifetime, so we can't access self in spawn
            let router = self.router.clone();
            tokio::spawn(async move {
                let mut conn = Conn::new(stream);
                handle_connection(router, &mut conn).await;
            });
        }
    }
}

async fn handle_connection(router: Router, conn: &mut Conn) {
    let mut req = Request::from(conn).await;
    info!("{} {}", req.method, req.full_path);
    let (params, handler) = router.dispatch(&req.path);
    req.params = params;
    let res = handler(req);

    conn.write_all(res.to_string().as_bytes()).await;
    conn.flush().await;
}

fn notfound(_req: Request) -> Response {
    Response::str("404 Not Found")
}
