use serde_json::json;
use web::{Application, Request, Response};

fn main() {
    let app = build_app();
    app.run();
}

fn build_app() -> Application {
    let mut app = Application::new("0:8080");
    app.route("/", index);
    app.route("/hello/:name", hello);

    app
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

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::build_app;

    #[test]
    fn it_works() {
        let app = build_app();
        let res = app.request("get", "/", HashMap::new(), &Vec::new());
        assert_eq!("Hello web.rs".as_bytes(), res.body());
    }
}
