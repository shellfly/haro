use web::{Application, Request, Response};

fn main() {
    let mut app = Application::new("0:8080");
    app.route("/", index);
    app.route("/hello/:name", hello);
    app.route("/template/:name", template);
    app.run();
}

fn index(_: Request) -> Response {
    Response::str("Hello web.rs")
}

fn hello(req: Request) -> Response {
    Response::json(req.params)
}

fn template(req: Request) -> Response {
    Response::tmpl("index.html", req.params)
}