# web.rs

web.rs is a web framework for Rust that is as simple as it is powerful.

Visit http://webrs.org/ for more information.

## Example

```Rust
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
```

``` bash
➜ curl localhost:1234
Hello web.rs
➜ curl localhost:1234/hello/world
{"name":"world"}
```
## Roadmap

- [x] query & headers
- [x] URL route
- [ ] Post
    - [ ] Forms
- [x] Response & JSON output
- [ ] Catch panic
- [ ] Templating
- [ ] Tests
- [ ] Static files
- [ ] Redirect
- [ ] Middleware
- [ ] Session and Cookie
- [ ] Database
- [ ] Deployment
