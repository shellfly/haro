use web::{Application, Request, Response};
fn main() {
    let mut app = Application::new("0:1234");
    app.route("/", index);
    app.run();
}

fn index(req: Request) -> Response {
    println!("{:?}", req);
    Response::from("hello web.rs")
}

fn hello(req: Request, name: String) -> Response {
    Response::from(&format!("hello {}", name))
}

struct Hello {}
impl Hello {
    fn get(&self, req: Request, name: String) -> Response {
        todo!()
    }
}
