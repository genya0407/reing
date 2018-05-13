#![feature(plugin)]
#![feature(custom_derive)]
#![plugin(rocket_codegen)]

extern crate dotenv;
extern crate chrono;
extern crate uuid;
extern crate rocket;
extern crate rocket_contrib;
extern crate base64;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;
#[macro_use]
extern crate diesel;
extern crate egg_mode;
extern crate tokio_core;
extern crate image;
extern crate imageproc;
extern crate rusttype;

use std::env;
use std::path::{Path, PathBuf};
use rocket::http::{Header, Status};
use rocket::request;
use rocket::response;
use rocket::response::status;
use rocket::Request;
use rocket_contrib::Template;
use chrono::prelude::*;

mod web;
mod model;
mod db;
mod text2image;
mod tweet;
mod utils;

#[derive(Serialize, Debug)]
struct AnswerDTO {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub created_at_recognizable: String,
}

impl AnswerDTO {
    fn from(a: model::Answer) -> Self {
        Self {
            id: a.id, body: a.body, created_at: a.created_at,
            created_at_recognizable: utils::recognizable_datetime(a.created_at)
        }
    }
}

#[derive(Serialize, Debug)]
struct QuestionDTO {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub hidden: bool,
    pub created_at: DateTime<Local>,
    pub created_at_recognizable: String,
    pub answers: Vec<AnswerDTO>
}

impl QuestionDTO {
    fn from(q: model::Question) -> Self {
        Self {
            id: q.id, body: q.body, ip_address: q.ip_address,
            hidden: q.hidden, created_at: q.created_at,
            answers: q.answers.into_iter().map(|a| AnswerDTO::from(a)).collect::<Vec<_>>(),
            created_at_recognizable: utils::recognizable_datetime(q.created_at)
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
struct IndexDTO {
    pub profile: ProfileDTO
}

#[derive(Serialize, Debug)]
struct ProfileDTO {
    pub username: String,
    pub image_url: String,
}

#[get("/")]
fn index() -> Template {
    let context = IndexDTO {
        profile: ProfileDTO {
            username: env::var("PROFILE_USERNAME").unwrap(),
            image_url: env::var("PROFILE_IMAGE_URL").unwrap()
        }
    };
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
    response::Redirect::to("/question/after_post")
}

/* GET /question/<id> */

#[derive(Serialize, Debug)]
struct AfterPostQuestionDTO{}

#[get("/question/after_post")]
fn after_post_question() -> Template {
    let context = AfterPostQuestionDTO{};
    Template::render("question/after_post", &context)
}

/* GET /admin */

#[derive(Serialize, Debug)]
struct AdminIndexDTO {
    pub questions: Vec<QuestionDTO>
}

#[get("/admin")]
fn admin_index(repo: web::guard::Repository, _auth: web::guard::BasicAuth) -> Template {
    let question_dtos = repo.not_answered_questions()
                            .into_iter()
                            .map(|q| QuestionDTO::from(q))
                            .collect::<Vec<_>>();
    let context = AdminIndexDTO { questions: question_dtos };
    Template::render("admin/index", &context)
}

/* GET /admin/question/<id> */

#[get("/admin/question/<id>")]
fn admin_show_question(id: i32, repo: web::guard::Repository, _auth: web::guard::BasicAuth) -> Template {
    let question = repo.find_question(id).unwrap();
    let context = QuestionDTO::from(question);
    Template::render("admin/questions/show", &context)
}

/* POST /question/<id>/answer */

#[derive(FromForm)]
struct PostAnswerForm {
    body: String
}

#[post("/admin/question/<id>/answer", data = "<params>")]
fn admin_post_answer(
    id: i32, repo: web::guard::Repository, 
    params: request::Form<PostAnswerForm>,
    _auth: web::guard::BasicAuth
    ) -> response::Redirect {

    let answer_body = params.get().body.clone();
    if let Some(question) = repo.store_answer(id, answer_body.clone()) {
        let img = text2image::text2image(question.body);
        tweet::tweet_answer(answer_body, img);
    }
    response::Redirect::to("/")
}

/* Force login */

struct RequireLogin();

impl<'r> response::Responder<'r> for RequireLogin {
    fn respond_to(self, _req: &Request) -> Result<response::Response<'r>, Status> {
        response::Response::build()
            .status(Status::Unauthorized)
            .header(Header::new("WWW-Authenticate", "Basic realm=\"SECRET AREA\""))
            .ok()
    }
}

#[error(401)]
fn unauthorized(_req: &Request) -> RequireLogin {
    RequireLogin()
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
        .mount("/", routes![
            index, files, post_question, after_post_question,
            admin_index, admin_post_answer, admin_show_question
        ])
        .catch(errors![unauthorized])
        .attach(Template::fairing())
        .launch();
}