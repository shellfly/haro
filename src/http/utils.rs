use std::collections::HashMap;

use log::{debug, info, warn};

use crate::http::conn::Conn;

pub fn read_headers(conn: &mut Conn) -> HashMap<String, String> {
    //TODO: limit headers
    let mut headers = HashMap::new();
    loop {
        let mut buf = String::new();
        conn.read_line(&mut buf);
        if buf == "\r\n" {
            break;
        }
        let header: Vec<&str> = buf.splitn(2, ':').collect();
        if header.len() != 2 {
            warn!("failed to parse header: {:?}", header);
        }
        headers.insert(header[0].trim().to_string(), header[1].trim().to_string());
    }
    headers
}

pub fn parse_query(query: Option<&str>) -> HashMap<String, String> {
    let mut args = HashMap::new();
    if let Some(query) = query {
        for q in query.split('&') {
            let qs: Vec<&str> = q.split('=').collect();
            if qs.len() != 2 {
                warn!("failed to parse query string: {:?}", qs);
                continue;
            }
            args.insert(qs[0].to_string(), qs[1].to_string());
        }
    }
    args
}

pub fn parse_json_body(body: &[u8]) -> HashMap<String, String> {
    serde_json::from_slice(body).unwrap()
}
