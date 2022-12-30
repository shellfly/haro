use std::{collections::HashMap, fmt::Display, fs, path::Path};

use http::{
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Response as HttpResponse, StatusCode,
};
use serde::Serialize;
use tinytemplate::TinyTemplate;

#[derive(Debug)]
pub struct Response {
    res: HttpResponse<Vec<u8>>,
}

impl Response {
    pub fn str<T: AsRef<str>>(s: T) -> Self {
        let body = s.as_ref().as_bytes().to_vec();
        let res = HttpResponse::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/plain")
            .header(CONTENT_LENGTH, body.len())
            .body(body)
            .unwrap();
        Self { res }
    }

    pub fn json<S>(s: S) -> Self
    where
        S: Serialize,
    {
        let s = serde_json::to_string(&s).unwrap();
        let body = s.as_bytes().to_vec();
        let res = HttpResponse::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "application/json")
            .header(CONTENT_LENGTH, body.len())
            .body(body)
            .unwrap();

        Self { res }
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

        let body = s.as_bytes().to_vec();
        let res = HttpResponse::builder()
            .status(StatusCode::OK)
            .header(CONTENT_TYPE, "text/html")
            .header(CONTENT_LENGTH, body.len())
            .body(body)
            .unwrap();

        Self { res }
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
