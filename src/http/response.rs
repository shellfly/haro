use std::{collections::HashMap, fmt::Display, fs, path::Path};

use http::{
    header::{HeaderName, CONTENT_LENGTH, CONTENT_TYPE, LOCATION},
    HeaderValue, Response as HttpResponse, StatusCode,
};
use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Debug)]
pub struct Response {
    res: HttpResponse<Vec<u8>>,
}

impl Response {
    pub fn new(status: StatusCode, body: &[u8], headers: HashMap<HeaderName, &str>) -> Self {
        let mut builder = HttpResponse::builder().status(status);
        for (key, val) in headers {
            builder = builder.header(key, val);
        }

        let res = builder
            .header(CONTENT_LENGTH, body.len())
            .body(body.to_vec())
            .unwrap();
        Self { res }
    }

    pub fn header<K, V>(self, key: K, value: V) -> Self
    where
        HeaderName: TryFrom<K>,
        <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
        HeaderValue: TryFrom<V>,
        <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
    {
        let (mut parts, body) = self.res.into_parts();
        let name = <HeaderName as TryFrom<K>>::try_from(key)
            .map_err(Into::into)
            .unwrap();
        let value = <HeaderValue as TryFrom<V>>::try_from(value)
            .map_err(Into::into)
            .unwrap();
        parts.headers.append(name, value);
        Self {
            res: HttpResponse::from_parts(parts, body),
        }
    }

    pub fn str<T: AsRef<str>>(s: T) -> Self {
        let body = s.as_ref().as_bytes();
        let headers = HashMap::from([(CONTENT_TYPE, "text/plain")]);
        Self::new(StatusCode::OK, body, headers)
    }

    pub fn json<S>(s: S) -> Self
    where
        S: Serialize,
    {
        let json_body = serde_json::to_vec(&s).unwrap();
        let body = json_body.as_ref();
        let headers = HashMap::from([(CONTENT_TYPE, "application/json")]);
        Self::new(StatusCode::OK, body, headers)
    }

    pub fn tmpl<T: AsRef<str>>(name: T, context: HashMap<String, String>) -> Self {
        // TODO(optimize): preload template
        let name = name.as_ref();
        let dir = env!("CARGO_MANIFEST_DIR");
        let file_path = Path::new(dir).join("src").join(name);
        let mut tt = TinyTemplate::new();
        let text = fs::read_to_string(file_path).unwrap();
        tt.add_template(name, &text).unwrap();
        let s = tt.render(name, &context).unwrap();

        let body = s.as_bytes();
        let headers = HashMap::from([(CONTENT_TYPE, "text/html")]);
        Self::new(StatusCode::OK, body, headers)
    }
}

impl Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write version and status
        let version = self.res.version();
        let status = self.res.status();
        write!(f, "{version:?} {status}\r\n").unwrap();

        // write headers
        for (key, val) in self.res.headers() {
            let val = std::str::from_utf8(val.as_bytes()).unwrap();
            write!(f, "{key}: {val}\r\n").unwrap();
        }

        // write body
        write!(f, "\r\n{}", std::str::from_utf8(self.res.body()).unwrap())
    }
}

pub fn redirect(location: &str, permanently: bool) -> Response {
    let status = if permanently {
        StatusCode::MOVED_PERMANENTLY
    } else {
        StatusCode::FOUND
    };
    let body = Vec::new();
    let headers = HashMap::from([(LOCATION, location)]);
    Response::new(status, &body, headers)
}
