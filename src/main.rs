#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate chrono;

use std::env;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use chrono::prelude::*;

type PostgresPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;
type PostgresConn = r2d2::PooledConnection<r2d2_postgres::PostgresConnectionManager>;

struct DbConn(pub PostgresConn);

impl Deref for DbConn {
    type Target = PostgresConn;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for DbConn {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PostgresPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(DbConn(conn)),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

#[get("/")]
fn index(conn: DbConn) -> &'static str {
    let rows = conn.query(
        "INSERT INTO questions (body, ip_address) values ($1, $2) returning id, created_at",
        &[&"bodybody".to_string(), &"192.168.1.1".to_string()]
    ).unwrap();
    let id: i32 = rows.iter().next().unwrap().get("id");
    let created_at: DateTime<Local> = rows.iter().next().unwrap().get("created_at");
    println!("{}", id);
    println!("{}", created_at);
    "Hello, world!"
}

fn main() {
    dotenv::dotenv().ok();
    let manager = r2d2_postgres::PostgresConnectionManager::new(
        env::var("DATABASE_URL").unwrap(),
        r2d2_postgres::TlsMode::None
    ).unwrap();
    let pool = r2d2::Pool::builder()
        .max_size(15)
        .build(manager)
        .unwrap();

    rocket::ignite()
        .manage(pool)
        .mount("/", routes![index])
        .launch();
}