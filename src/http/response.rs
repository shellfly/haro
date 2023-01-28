use std::{collections::HashMap, fmt::Display};

use http::{
    header::{HeaderName, CONTENT_LENGTH, CONTENT_TYPE, LOCATION},
    HeaderValue, Response as HttpResponse, StatusCode,
};
use log::error;
use serde::Serialize;

#[cfg(feature = "template")]
use crate::template::TEMPLATES;
#[cfg(feature = "template")]
use tera::Context;

/// HTTP Response
#[derive(Debug)]
pub struct Response {
    res: HttpResponse<Vec<u8>>,
}

impl Response {
    /// Create a new `Response`
    /// # Examples
    /// ```
    /// use std::collections::HashMap;
    /// use http::StatusCode;
    /// use haro::Response;
    ///
    /// let body = &Vec::new();
    /// let headers = HashMap::new();
    /// let res = Response::new(StatusCode::OK, body, headers);
    /// ```
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

    /// Returns status code of the `Response`
    pub fn status(&self) -> StatusCode {
        self.res.status()
    }

    /// Returns body of the `Response`
    pub fn body(&self) -> &[u8] {
        self.res.body()
    }

    /// Set a new header, generate and return a new `Response`
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

    /// Generate plain text response
    /// # Example
    /// ```
    /// use haro::Response;
    ///
    /// let res = Response::str("Hello Haro");
    /// ```
    pub fn str<T: AsRef<str>>(s: T) -> Self {
        let body = s.as_ref().as_bytes();
        let headers = HashMap::from([(CONTENT_TYPE, "text/plain")]);
        Self::new(StatusCode::OK, body, headers)
    }

    /// Generate JSON response
    /// # Example
    /// ```
    /// use haro::Response;
    ///
    /// let res = Response::json("{\"hello\":\"world\"}");
    /// ```
    pub fn json<S>(s: S) -> Self
    where
        S: Serialize,
    {
        let json_body = serde_json::to_vec(&s).unwrap();
        let body = json_body.as_ref();
        let headers = HashMap::from([(CONTENT_TYPE, "application/json")]);
        Self::new(StatusCode::OK, body, headers)
    }

    /// Generate a response by a template
    /// # Example
    /// ```no_run
    /// use tera::Context;
    /// use haro::Response;
    ///
    /// let context = Context::new();
    /// let res = Response::tmpl("index.html", context);
    /// ```
    #[cfg(feature = "template")]
    pub fn tmpl(name: &str, context: Context) -> Self {
        let mut status = StatusCode::OK;
        let body = match TEMPLATES.render(name, &context) {
            Ok(body) => body,
            Err(error) => {
                error!(
                    "failed to render template: {}, context: {:?}, error: {}",
                    name, context, error
                );
                status = StatusCode::INTERNAL_SERVER_ERROR;
                String::new()
            }
        };

        let headers = HashMap::from([(CONTENT_TYPE, "text/html")]);
        Self::new(status, body.as_bytes(), headers)
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

/// redirect is a helper function to generate 301 or 302 [`Response`]
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
