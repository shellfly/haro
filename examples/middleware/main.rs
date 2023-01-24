use std::sync::Arc;

use serde_json::json;
use web::{
    middleware::{self, DynHandler},
    Application, Request, Response,
};

fn main() {
    let mut app = Application::new("0:8080");
    app.middleware(middleware::logging);
    app.middleware(my_middleware);
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

fn my_middleware(next: DynHandler) -> DynHandler {
    Arc::new(move |req: Request| -> Response {
        println!("before request");
        let res = next(req);
        println!("after request");
        res
    })
}
