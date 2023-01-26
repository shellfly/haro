# Haro

**Haro** is a **simple** and **synchronous** web framework written in and for Rust.

The project was named after the [Haro character](https://en.wikipedia.org/wiki/Haro_(character)). The application interface was inspired by the [web.py](https://webpy.org/) project.

## Motivation
> In short, async Rust is more difficult to use and can result in a higher maintenance burden than synchronous Rust, but gives you best-in-class performance in return. All areas of async Rust are constantly improving, so the impact of these issues will wear off over time
>
> https://rust-lang.github.io/async-book/01_getting_started/03_state_of_async_rust.html

As the async book says, async Rust is not mature yet. While bringing performance, it also results in a higher maintenance burden. The goal of this project is to create a simple and minimum synchronous Web framework for Rust.

## Quick Start

Add `haro` as a dependency by cargo
```bash
cargo add haro
```

Then, on your main.rs:

```Rust
use haro::{Application, Request, Response};
use serde_json::json;

fn main() {
    let mut app = Application::new("0:8000");
    app.route("/", index);
    app.route("/hello/:name", hello);
    app.run();
}

fn index(_: Request) -> Response {
    Response::str("Hello Haro")
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

Hello Haro
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

## More Examples

The repo contains [more examples](./examples) that show how to put all the pieces together.

## Features

- [x] URL Routing with plain fn pointer
- [x] Request & Response with minimal boilerplate
  - [x] Query args
  - [x] Post data
  - [x] JSON
  - [x] Cookie
- [x] Middleware
- [x] Template (Optional)
- [x] Database (Optional)
- [x] Tests
- [ ] HTTP2



## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
