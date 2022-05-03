use web::{Application, Request, Response};
fn main() {
    let mut app = Application::new("0:1234");
    app.route("/", index);
    app.route("/hello/:name", hello);
    app.run();
}

fn index(_req: Request) -> Response {
    Response::str("Hello web.rs")
}

fn hello(req: Request) -> Response {
    Response::json(req.params)
}
