use haro::{Application, Handler, Request, Response};

fn main() {
    let mut app = Application::new("0:8080");
    let hello_handler = HelloHandler {
        name: "Haro".to_string(),
    };
    app.route("/", index);
    app.route_handler("/hello", hello_handler);
    app.run();
}

fn index(_: Request) -> Response {
    Response::str("Hello Haro")
}
struct HelloHandler {
    name: String,
}

impl Handler for HelloHandler {
    fn call(&self, _: Request) -> Response {
        Response::str(format!("hello {}", self.name))
    }
}
