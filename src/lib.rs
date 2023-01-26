//! # web
//!
//! web is a **simple** and **synchronous** web framework written in and for Rust.
//!
//! ## Features
//!
//! - Routing with plain fn pointer
//! - Request & Response with minimal boilerplate
//!   - Query args
//!   - Post data
//!   - JSON
//!   - Cookie
//! - Middleware
//! - Database
//! - Tests
//!

mod app;
pub mod db;
mod http;
pub mod middleware;
mod pool;
mod router;

pub use crate::app::Application;
pub use crate::http::request::Request;
pub use crate::http::response::{redirect, Response};
