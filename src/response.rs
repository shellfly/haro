use std::{collections::HashMap, fmt::Display};

use serde_json;

#[derive(Debug)]
pub struct Response {
    pub status_code: u8, //TODO: enum
    pub status: String,
    pub content: String,
    pub content_type: String,
    pub content_length: usize,
}

impl Response {
    pub fn str<T: AsRef<str>>(s: T) -> Self {
        Self {
            status_code: 200,
            status: "OK".to_string(),
            content: s.as_ref().to_string(),
            content_type: "text/plain".to_string(),
            content_length: s.as_ref().len(),
        }
    }

    pub fn json(s: HashMap<String, String>) -> Self {
        let s = serde_json::to_string(&s).unwrap();
        Self {
            status_code: 200,
            status: "OK".to_string(),
            content_length: s.len(),
            content: s,
            content_type: "application/json".to_string(),
        }
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "HTTP/1.1 {} {}\r\nContent-Length: {}\r\nContent-Type: {}\r\n\r\n{}",
            self.status_code, self.status, self.content_length, self.content_type, self.content,
        )
    }
}
