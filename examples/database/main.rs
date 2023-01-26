// NOTE: add `web` crate with `full` or `database` feature flag to use database utilities.

use mysql::prelude::*;
use serde::{Deserialize, Serialize};
use web::{db, Application, Request, Response};

fn main() {
    db::Postgres::init("postgres://postgres:postgres@localhost:5432/test");
    db::MySQL::init("mysql://root:root@localhost:3306/test");
    db::SQLite::init("test.db");

    let mut app = Application::new("0:8080");
    app.route("/pg", pg);
    app.route("/my", my);
    app.route("/sqlite", sqlite);
    app.run();
}

#[derive(Serialize, Deserialize)]
struct Person {
    id: i32,
    name: String,
}

fn pg(_: Request) -> Response {
    let mut client = db::Postgres::get();

    // create table
    client
        .batch_execute(
            "
    CREATE TABLE IF NOT EXISTS person (
        id      SERIAL PRIMARY KEY,
        name    TEXT NOT NULL
    )
",
        )
        .unwrap();

    // insert data
    let name = "Ferris";
    client
        .execute("INSERT INTO person (name) VALUES ($1)", &[&name])
        .unwrap();

    // select data
    let mut persons = Vec::new();
    for row in client.query("SELECT id, name FROM person", &[]).unwrap() {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        persons.push(Person {
            id,
            name: name.to_string(),
        });
    }

    Response::json(persons)
}

fn my(_: Request) -> Response {
    let mut conn = db::MySQL::get();

    // create table
    conn.query_drop(
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
    conn.exec_drop("INSERT INTO person (name) VALUES (?)", (name,))
        .unwrap();

    // select data
    let stmt = conn.prep("SELECT id, name FROM person").unwrap();
    let persons = conn
        .exec_map(stmt, (), |(id, name)| Person { id, name })
        .unwrap();

    Response::json(persons)
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
