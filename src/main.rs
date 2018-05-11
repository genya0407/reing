#![feature(plugin)]
#![plugin(rocket_codegen)]

extern crate rocket;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate chrono;
extern crate reing;

use std::env;
use std::ops::Deref;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
//use chrono::prelude::*;

type PostgresPool = r2d2::Pool<r2d2_postgres::PostgresConnectionManager>;

struct Repository(pub reing::Repository);

impl Deref for Repository {
    type Target = reing::Repository;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for Repository {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        let pool = request.guard::<State<PostgresPool>>()?;
        match pool.get() {
            Ok(conn) => Outcome::Success(Repository(reing::Repository::new(conn))),
            Err(_) => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

#[get("/")]
fn index(repository: Repository) -> &'static str {
    let question = repository.store_question("testtest".to_string(), "192.168.1.1".to_string());
    println!("{:?}", question);
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