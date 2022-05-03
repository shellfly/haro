use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

use log::warn;

pub fn parse_headers(reader: &mut BufReader<TcpStream>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    loop {
        let mut buf = String::new();
        reader.read_line(&mut buf).unwrap();
        if buf == "\r\n" {
            break;
        }
        let header: Vec<&str> = buf.trim().splitn(2, ":").collect();
        if header.len() != 2 {
            warn!("failed to parse header: {:?}", header);
        }
        headers.insert(header[0].trim().to_string(), header[1].trim().to_string());
    }
    headers
}

pub fn parse_query(query: &str) -> HashMap<String, String> {
    let mut get = HashMap::new();
    for q in query.split("&") {
        let qs: Vec<&str> = q.split("=").collect();
        if qs.len() != 2 {
            warn!("failed to parse query string: {:?}", qs);
            continue;
        }
        get.insert(qs[0].to_string(), qs[1].to_string());
    }
    get
}
pub fn parse_body(reader: &mut BufReader<TcpStream>) -> HashMap<String, String> {
    HashMap::new()
}
