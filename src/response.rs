use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

use log::{debug, error, info};

#[derive(Debug)]
pub struct Response {
    pub status_code: u8, //TODO: enum
    pub status: String,
    pub content: String,
    pub content_length: usize,
}

impl Response {
    pub fn from(s: &str) -> Self {
        Self {
            status_code: 200,
            status: "OK".to_string(),
            content: s.to_string(),
            content_length: s.len(),
        }
    }
}
