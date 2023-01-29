use haro::{Application, Handler, Request, Response};
use serde_json::json;

fn main() {
    let mut app = Application::new("0:8080");
    let hello_handler = HelloHandler {
        name: "Haro".to_string(),
    };
    app.route("/", |_| Response::str("Hello Haro")); // route by closure
    app.route("/input/:name", input); // route by function
    app.route_handler("/hello", hello_handler); //route by `Handler` trait type
    app.run();
}

fn input(req: Request) -> Response {
    let data = json!({
        "method":req.method(),
        "args":req.args,
        "params":req.params,
        "data":req.data,
    });
    Response::json(data)
}

struct HelloHandler {
    name: String,
}

impl Handler for HelloHandler {
    fn call(&self, _: Request) -> Response {
        Response::str(format!("hello {}", self.name))
    }
}
