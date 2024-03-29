# Haro

![ci](https://github.com/shellfly/haro/actions/workflows/ci.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/haro.svg)](https://crates.io/crates/haro)
[![chat](https://img.shields.io/badge/chat-discord-brightgreen)](https://discord.gg/AktRUJKphe)

**Haro** is a **simple** and **synchronous** web framework written in and for Rust.

The project was named after the [Haro character](https://en.wikipedia.org/wiki/Haro_(character)). The application interface was inspired by the [web.py](https://webpy.org/) project.

## Motivation
> In short, async Rust is more difficult to use and can result in a higher maintenance burden than synchronous Rust, but gives you best-in-class performance in return. All areas of async Rust are constantly improving, so the impact of these issues will wear off over time
>
> https://rust-lang.github.io/async-book/01_getting_started/03_state_of_async_rust.html

As the async book says, async Rust is not mature yet. While bringing performance, it also results in a higher maintenance burden. The goal of this project is to create a simple and minimum synchronous Web framework for Rust.

## Features

- [x] URL Routing with **function**/**closure**/**trait type**
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

## Quick Start

Add `haro` as a dependency by cargo
```bash
cargo add haro
```

Then, on your main.rs:

```rust
use haro::{Application, Request, Response};

fn main() {
    let mut app = Application::new("0:8080");
    app.route("/", hello);
    app.run();
}

fn hello(_:Request) ->Response{
    Response::str("Hello Haro")
}
```

## Route by closure

Use a closure as a handler function to capture environment variables

```rust
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
```

## `Handler` trait

The handler cloud be a struct that implements the `Handler` trait to handle a request.
```rust
use haro::{Application, Request, Response, Handler};

fn main() {
    let mut app = Application::new("0:8080");
    let hello = HelloHandler {
        name: "Haro".to_string(),
    };
    app.route_handler("/", hello);
    app.run();
}

struct HelloHandler {
    name: String,
}

impl Handler for HelloHandler {
    fn call(&self, _: Request) -> Response {
        Response::str(format!("hello {}", self.name))
    }
}
```

## JSON response

```rust
use haro::{Application, Request, Response};
use serde_json::json;

...

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

## More examples

The repo contains [more examples](./examples) that show how to put all the pieces together.





## License

Licensed under either of

 * Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
