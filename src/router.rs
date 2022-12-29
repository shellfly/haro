use std::collections::HashMap;
use std::fmt::Display;

use regex::Regex;

use crate::request::Request;
use crate::response::Response;

pub type Handler = fn(Request) -> Response;

#[derive(Default, Clone)]
pub struct Router {
    routes: Vec<(Rule, Handler)>,
}

impl Router {
    pub fn add(&mut self, pattern: &'static str, f: Handler) {
        let rule = Rule::from(pattern);
        self.routes.push((rule, f));
        self.update_order()
    }

    fn update_order(&mut self) {
        // TODO: compare performance with a radix tree
        self.routes
            .sort_by(|a, b| b.0.num_parts.cmp(&a.0.num_parts));
    }

    pub fn dispatch(&self, path: &str) -> (HashMap<String, String>, Handler) {
        for (rule, handler) in &self.routes {
            if let Some(params) = rule._match(path) {
                return (params, *handler);
            }
        }
        (HashMap::new(), not_found)
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

impl Display for Rule {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.pattern)
    }
}

impl From<&'static str> for Rule {
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
}

impl Rule {
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

fn not_found(_req: Request) -> Response {
    Response::str("404 Not Found")
}
