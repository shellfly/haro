use std::collections::HashMap;

use log::warn;

use crate::conn::Conn;

pub fn parse_headers(conn: &mut Conn) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    loop {
        let mut buf = String::new();
        conn.read_line(&mut buf);
        if buf == "\r\n" {
            break;
        }
        let header: Vec<&str> = buf.trim().splitn(2, ':').collect();
        if header.len() != 2 {
            warn!("failed to parse header: {:?}", header);
        }
        headers.insert(header[0].trim().to_string(), header[1].trim().to_string());
    }
    headers
}

pub fn parse_query(query: &str) -> HashMap<String, String> {
    let mut get = HashMap::new();
    for q in query.split('&') {
        let qs: Vec<&str> = q.split('=').collect();
        if qs.len() != 2 {
            warn!("failed to parse query string: {:?}", qs);
            continue;
        }
        get.insert(qs[0].to_string(), qs[1].to_string());
    }
    get
}
pub fn parse_body(conn: &mut Conn) -> HashMap<String, String> {
    HashMap::new()
}
