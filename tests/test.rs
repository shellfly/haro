use haro::{
    db,
    middleware::{self, DynHandler},
    Application, Request, Response,
};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::{collections::HashMap, sync::Arc};

#[test]
fn test_app() {
    db::SQLite::init("test.db");

    let mut app = Application::new("0:8080");
    app.middleware(middleware::logging);
    app.middleware(my_middleware);
    app.route("/", index);
    app.route("/hello/:name", hello);
    app.route("/input", input);
    app.route("/sqlite", sqlite);

    let res = app.request("get", "/", HashMap::new(), &Vec::new());
    assert_eq!("Hello Haro".as_bytes(), res.body());

    let res = app.request("get", "/hello/world", HashMap::new(), &Vec::new());
    assert_eq!("{\"name\":\"world\"}".as_bytes(), res.body());

    let body = "{\"name\":\"Haro\"}";
    let res = app.request("post", "/input?a=b", HashMap::new(), body.as_bytes());
    assert_eq!(StatusCode::OK, res.status());

    let res = app.request("get", "/sqlite", HashMap::new(), &Vec::new());
    let persons: Vec<Person> = serde_json::from_slice(res.body()).unwrap();
    assert_eq!("Ferris", persons[0].name);
}

#[derive(Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
}

fn index(_: Request) -> Response {
    Response::str("Hello Haro")
}

fn hello(req: Request) -> Response {
    let data = json!({
        "name":req.params["name"],
    });
    Response::json(data)
}

fn input(req: Request) -> Response {
    let data = json!({
        "args":req.args,
        "data":req.data,
    });
    Response::json(data)
}

fn sqlite(_: Request) -> Response {
    let conn = db::SQLite::get();

    // create table
    conn.execute_batch(
        "
    CREATE TABLE IF NOT EXISTS person (
        id      INTEGER PRIMARY KEY,
        name    TEXT NOT NULL
    )
",
    )
    .unwrap();

    // insert data
    let name = "Ferris";
    conn.execute("INSERT INTO person (name) VALUES ($1)", [&name])
        .unwrap();

    // select data
    let mut stmt = conn.prepare("SELECT id, name FROM person").unwrap();
    let persons: Vec<Person> = stmt
        .query_map([], |row| {
            Ok(Person {
                id: row.get(0)?,
                name: row.get(1)?,
            })
        })
        .unwrap()
        .flatten()
        .collect();

    Response::json(persons)
}

fn my_middleware(next: DynHandler) -> DynHandler {
    Arc::new(move |req: Request| -> Response {
        println!("before request");
        let res = next(req);
        println!("after request");
        res
    })
}
