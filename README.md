# web.rs

web.rs is a web framework for Rust that is as simple as it is powerful.

Visit http://webrs.org/ for more information.

## Example

```Rust
use web::{Application, Request, Response};

fn main() {
    let mut app = Application::new("0:8000");
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
```

``` bash
➜ curl localhost:8000
Hello web.rs
➜ curl localhost:8000/hello/world
{"name":"world"}
```
## Road map

- [x] query & headers
- [x] URL route
- [x] Template
- [ ] Post
    - [ ] Forms
    - [x] JSON
- [x] Response & JSON output
- [ ] hyper request & response
- [ ] Thread pool
- [ ] Catch panic
- [ ] Tests
    - [ ] self tests
    - [ ] app tests
- [ ] Static files
- [ ] Redirect
- [ ] Middleware
- [ ] Session and Cookie
- [ ] HTTP2
- [ ] Database
- [ ] Deployment

