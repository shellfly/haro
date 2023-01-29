use haro::{Application, Request, Response};

fn main() {
    let mut app = Application::new("0:8080");
    app.route("/", |_| Response::str("Haro"));
    app.route("/hello", hello("Haro"));
    app.run();
}

fn hello(name: &str) -> impl Fn(Request) -> Response {
    let name = name.to_string();
    move |_: Request| Response::str(&name)
}


