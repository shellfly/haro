/// Use `route_dyn` to register a handler with closure
use std::sync::Arc;

use haro::{Application, DynHandler, Response};

fn main() {
    let mut app = Application::new("0:8080");
    app.route_dyn("/", hello("Haro"));
    app.run();
}

fn hello(name: &str) -> DynHandler {
    let name = name.to_string();
    Arc::new(move |_| Response::str(&name))
}
