//! # web
//!
//! web is a **simple** and **synchronous** web framework written in and for Rust.
//!
//! ## Features
//!
//! - URL Routing with plain fn pointer
//! - Request & Response with minimal boilerplate
//!   - Query args
//!   - Post data
//!   - JSON
//!   - Cookie
//! - Middleware
//! - Template (optional)
//! - Database (optional)
//! - Tests
//!
//! ## Example
//!
//! The "Hello, World!" of web is:
//!
//! ```rust,no_run
//! use web::{Applicatoin,  Request, Response}
//!
//! fn main() {
//!     let mut app = Application::new("0:8000");
//!     app.route("/", hello);
//!     app.run();
//! }
//!
//! fn hello(_: Request) -> Response {
//!     Response::str("Hello web.rs")
//! }
//! ```
//!
//! ## Optional Features
//!
//! web uses a set of [feature flags] to reduce the amount of compiled code and
//! optional dependencies.
//!
//! You can also use the `full` feature flag which will enable all public APIs.
//! Beware that this will pull in many extra dependencies that you may not need.
//!
//! The following optional features are available:
//!
//! - `database`: Enables Database support.
//! - `template`: Enables Template support.
//!
//! [feature flags]: https://doc.rust-lang.org/cargo/reference/manifest.html#the-features-section
//!
//! ## Examples
//!
//! The web repo contains [a number of examples][examples] that show how to put all the
//! pieces together.
//!
//! [examples]: https://github.com/shellfly/web.rs/tree/main/examples
//!
mod app;
mod http;
pub mod middleware;
mod pool;
mod router;

pub use crate::app::Application;
pub use crate::http::request::Request;
pub use crate::http::response::{redirect, Response};

#[cfg(feature = "template")]
mod template;

#[cfg(feature = "database")]
pub mod db;
