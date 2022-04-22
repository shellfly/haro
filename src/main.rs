use web::Application;
fn main() {
    // TODO: move logger to lib.rs
    env_logger::init();
    let mut app = Application::new("0:1234");
    app.route("/", index);
    app.route("/hello", hello);
    app.run();
}

fn index() -> String {
    "hello web.rs".to_string()
}
fn hello() -> String {
    "hello world".to_string()
}
