use std::sync::Arc;

use cookie::Cookie;
use http::header::{COOKIE, SET_COOKIE};
use serde_json::json;
use web::{
    middleware::{self, DynHandler},
    Application, Request, Response,
};

fn main() {
    let mut app = Application::new("0:8080");
    app.middleware(middleware::logging);
    app.middleware(session_middleware);
    app.route("/", index);
    app.route("/hello/:name", hello);
    app.run();
}

fn index(_: Request) -> Response {
    Response::str("Hello web.rs")
}

fn hello(req: Request) -> Response {
    let data = json!({
        "method":req.method(),
        "args":req.args,
        "params":req.params,
        "data":req.data,
    });
    Response::json(data)
}

fn session_middleware(next: DynHandler) -> DynHandler {
    Arc::new(move |req: Request| -> Response {
        let cookies = req.cookies();
        println!("get cookies: {:?}", cookies);

        let mut res = next(req);

        let cookie1 = Cookie::build("foo", "bar").finish();
        let cookie2 = Cookie::build("bar", "baz")
            .domain("example.coom")
            .path("/")
            .secure(true)
            .http_only(true)
            .finish();
        res = res.header(SET_COOKIE, &cookie1.to_string());
        res.header(SET_COOKIE, &cookie2.to_string())
    })
}
