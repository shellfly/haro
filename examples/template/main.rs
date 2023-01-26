use tera::Context;
use web::{Application, Request, Response};

fn main() {
    let mut app = Application::new("0:8080");

    app.route("/", index);
    app.route("/hello/:name", tmpl);
    app.run();
}

fn index(_: Request) -> Response {
    Response::str("Hello web.rs")
}

fn tmpl(req: Request) -> Response {
    let mut context = Context::new();
    context.insert("name", &req.params["name"]);
    context.insert("number", &10);
    Response::tmpl("index.html", context)
}
