# web.rs

web.rs is a web framework for Rust that is as simple as it is powerful.

Visit http://webrs.org/ for more information.

## Example

```Rust
use web::Application;
fn main() {
    let mut app = Application::new("0:1234");
    app.route("/", index);
    app.run();
}

fn index() -> String {
    "hello web.rs".to_string()
}
```
## Roadmap

- [x] query & headers
- [ ] URL route with regexp
- [ ] Post
    - [ ] Forms
- [ ] Response & JSON output
- [ ] Templating
- [ ] Tests
- [ ] Static files
- [ ] Redirect
- [ ] Middleware
- [ ] Session and Cookie
- [ ] Database
- [ ] Deployment
