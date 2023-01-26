use haro::{Application, Request, Response};
use serde_json::json;

fn main() {
    let mut app = Application::new("0:8080");

    app.route("/", index);
    app.route("/hello/:name", hello);
    app.run();
}

fn index(_: Request) -> Response {
    Response::str("Hello Haro")
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
