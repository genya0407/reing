#![feature(plugin)]
#![feature(custom_derive)]
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
use std::net::SocketAddr;
use rocket::http::Status;
use rocket::request::{self, FromRequest};
use rocket::{Request, State, Outcome};
use rocket::response;
use rocket::response::status;
use rocket_contrib::Template;
use chrono::prelude::*;

/* Guard Repository */

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

/* Guard Client IP address */

struct ClientIP(pub SocketAddr);

impl ClientIP {
    fn address(&self) -> String {
        match self.0 {
            SocketAddr::V4(v4) => format!("{}", v4.ip()),
            SocketAddr::V6(v6) => format!("{}", v6.ip())
        }
    }
}

impl<'a, 'r> FromRequest<'a, 'r> for ClientIP {
    type Error = ();

    fn from_request(request: &'a Request<'r>) -> request::Outcome<Self, Self::Error> {
        match request.remote() {
            Some(socket_addr) => Outcome::Success(ClientIP(socket_addr)),
            None => Outcome::Failure((Status::ServiceUnavailable, ()))
        }
    }
}

/* GET /static/ */

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Result<response::NamedFile, status::NotFound<String>> {
    let path = Path::new("static/").join(file);
    response::NamedFile::open(&path)
        .map_err(|_| status::NotFound(format!("Bad path: {:?}", path)))
}

/* GET / */

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

/* POST /questions */

#[derive(FromForm)]
struct PostQuestionForm {
    body: String
}

#[post("/questions", data = "<params>")]
fn post_question(repo: Repository, client_ip: ClientIP, params: request::Form<PostQuestionForm>)
     -> response::Redirect {
    let _question = repo.store_question(params.get().body.clone(), client_ip.address());
    //response::Redirect::to(&format!("/question/{}", question.id))
    response::Redirect::to("/")
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
        .mount("/", routes![index, files, post_question])
        .attach(Template::fairing())
        .launch();
}