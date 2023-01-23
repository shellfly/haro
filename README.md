# web.rs

web.rs is a web framework for Rust that is as simple as it is powerful.

Visit https://webrs.org/ for more information.

## Motivation
> In short, async Rust is more difficult to use and can result in a higher maintenance burden than synchronous Rust, but gives you best-in-class performance in return. All areas of async Rust are constantly improving, so the impact of these issues will wear off over time
>
> https://rust-lang.github.io/async-book/01_getting_started/03_state_of_async_rust.html

As the async book says, while bringing performance, async Rust can result in a higher maintenance burden. The goal of this project is to create a simple and minimum synchronous Web framework for Rust.

## Example

Add `web` as a dependency by cargo
```bash
cargo add web
```

Then, on your main.rs:

```Rust
use web::{Application, Request, Response};

fn main() {
    let mut app = Application::new("0:8000");
    app.route("/", index);
    app.route("/hello/:name", hello);
    app.run();
}

fn index(_: Request) -> Response {
    Response::str("Hello web.rs")
}

fn hello(req: Request) -> Response {
    let data = json!({
        "method":req.method(),
        "args":req.args,
        "params":req.params,
        "data":req.data,
    });
    Response::json(data)
}
```

```bash
http get "localhost:8080/"
HTTP/1.1 200 OK
content-length: 12
content-type: text/plain

Hello web.rs
```

```bash
http post "localhost:8080/hello/world?a=b" c=d
HTTP/1.1 200 OK
content-length: 77
content-type: application/json

{
    "args": {
        "a": "b"
    },
    "data": {
        "c": "d"
    },
    "method": "POST",
    "params": {
        "name": "world"
    }
}
```
## Road map

- [x] query & headers
- [x] URL route
- [x] Template
- [ ] Post
    - [ ] Forms
    - [x] JSON
- [x] Response & JSON output
- [ ] Thread pool
- [ ] Catch panic
- [ ] Tests
    - [ ] self tests
    - [ ] app tests
- [ ] Static files
- [ ] Redirect
- [ ] Middleware
- [ ] Session and Cookie
- [ ] Database
- [ ] Deployment

