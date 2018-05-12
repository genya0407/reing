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
extern crate r2d2_diesel;
extern crate chrono;
extern crate query_builder;
extern crate egg_mode;
extern crate tokio_core;
#[macro_use]
extern crate diesel;

use std::env;
use std::path::{Path, PathBuf};
use rocket::request;
use rocket::response;
use rocket::response::status;
use rocket_contrib::Template;
use chrono::prelude::*;

mod web;
mod model;
mod db;
//mod tweet;

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
    fn from(q: model::Question) -> Self {
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
fn index(repo: web::guard::Repository) -> Template {
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
fn post_question(repo: web::guard::Repository, client_ip: web::guard::ClientIP, params: request::Form<PostQuestionForm>)
     -> response::Redirect {
    let _question = repo.store_question(params.get().body.clone(), client_ip.address());
    //response::Redirect::to(&format!("/question/{}", question.id))
    response::Redirect::to("/")
}

fn main() {
    dotenv::dotenv().ok();
    let manager = r2d2_diesel::ConnectionManager::<diesel::PgConnection>::new(
        env::var("DATABASE_URL").unwrap()
    );
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