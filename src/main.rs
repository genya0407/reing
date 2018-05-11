#![feature(plugin)]
#![plugin(rocket_codegen)]

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rocket;
extern crate rocket_contrib;
extern crate dotenv;
extern crate r2d2;
extern crate r2d2_postgres;
extern crate chrono;
extern crate reing;

use std::env;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket::response;
use rocket::response::status;
use rocket_contrib::Template;
use chrono::prelude::*;

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

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Result<response::NamedFile, status::NotFound<String>> {
    let path = Path::new("static/").join(file);
    response::NamedFile::open(&path)
        .map_err(|_| status::NotFound(format!("Bad path: {:?}", path)))
}

#[derive(Serialize, Debug)]
struct QuestionDTO {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub hidden: bool,
    pub created_at: DateTime<Local>,
}

impl QuestionDTO {
    fn from(q: reing::Question) -> Self {
        Self {
            id: q.id, body: q.body, ip_address: q.ip_address,
            hidden: q.hidden, created_at: q.created_at
        }
    }
}


#[derive(Serialize, Debug)]
struct IndexDTO {
    pub questions: Vec<QuestionDTO>
}

#[get("/")]
fn index(repo: Repository) -> Template {
    let question_dtos = repo.all_questions().into_iter().map(|q| QuestionDTO::from(q)).collect::<Vec<_>>();
    let context = IndexDTO { questions: question_dtos };
    Template::render("index", &context)
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
        .attach(Template::fairing())
        .launch();
}