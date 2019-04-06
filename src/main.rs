#![feature(proc_macro_hygiene, decl_macro)]

extern crate dotenv;
extern crate chrono;
extern crate uuid;
#[macro_use] extern crate rocket;
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
extern crate tokio;
extern crate lettre;
extern crate lettre_email;
extern crate htmlescape;
extern crate reing_text2image;

use std::sync::mpsc::{SyncSender, sync_channel};
use std::env;
use std::path::{Path, PathBuf};
use rocket::http::{Header, Status};
use rocket::request;
use rocket::response;
use rocket::response::status;
use rocket::Request;
use rocket_contrib::templates::Template;
use chrono::prelude::*;
use reing_text2image::TextImage;
use std::{thread, time};
use rocket::State;

mod web;
mod model;
mod db;
mod tweet;
mod utils;
mod notify;

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

/* Force ssl */
#[get("/<path..>")]
fn redirect_ssl(path: PathBuf, _ssl: web::guard::ForceSSL) -> response::Redirect {
    let redirect_to = format!("https://{}/{}", env::var("APPLICATION_DOMAIN").unwrap(), path.as_path().display());
    println!("Redirect to:{}", redirect_to);
    response::Redirect::to(redirect_to)
}

/* GET /static/ */

#[get("/static/<file..>")]
fn files(file: PathBuf) -> Result<web::CachedFile, status::NotFound<String>> {
    let path = Path::new("static/").join(file);
    response::NamedFile::open(&path)
        .map_err(|_| status::NotFound(format!("Bad path: {:?}", path)))
        .map(|nf| web::CachedFile(nf))
}

/* GET / */

#[derive(Serialize, Debug)]
struct IndexDTO {
    pub profile: ProfileDTO,
    pub answered_questions: Vec<QuestionDTO>,
    pub site_url: String,
    pub next_page: Option<i64>,
    pub prev_page: Option<i64>,
}

#[derive(Serialize, Debug)]
struct ProfileDTO {
    pub username: String,
    pub image_url: String,
}

#[test]
fn next_prev_page_test() {
    assert!((None, Some(1)) == next_prev_page(0));
    assert!((Some(0), Some(2)) == next_prev_page(1));
    assert!((Some(1), Some(3)) == next_prev_page(2));
}

// next: newer, prev: older
// older -> page number increases
fn next_prev_page(current_page: i64) -> (Option<i64>, Option<i64>) {
    let prev_page = Some(current_page + 1);
    let next_page = if current_page <= 0 {
        None
    } else {
        Some(current_page - 1)
    };
    return (next_page, prev_page);
}

const QUESTION_COUNT_PER_PAGE : i64 = 30;
#[get("/")]
fn index(repo: web::guard::Repository, profile: State<UserProfile>) -> Template {
    let page = 0;
    index_with_page(repo, profile, page)
}

#[get("/page/<page>")]
fn index_with_page(repo: web::guard::Repository, profile: State<UserProfile>, page: i64) -> Template {
    let offset = page * QUESTION_COUNT_PER_PAGE;
    let mut question_dtos = repo.answered_questions(offset, QUESTION_COUNT_PER_PAGE)
                                .into_iter()
                                .map(|q| QuestionDTO::from(q))
                                .collect::<Vec<_>>();
    question_dtos.reverse();
    let (next_page, prev_page) = next_prev_page(page);
    let context = IndexDTO {
        profile: ProfileDTO {
            username: profile.clone().name,
            image_url: String::from("/static/image/profile.jpg")
        },
        answered_questions: question_dtos,
        site_url: format!("https://{}/", env::var("APPLICATION_DOMAIN").unwrap()),
        prev_page: prev_page,
        next_page: next_page,
    };
    Template::render("index", &context)
}

#[derive(Serialize, Debug)]
struct SearchDTO {
    pub profile: ProfileDTO,
    pub search_results: Vec<QuestionDTO>,
    pub site_url: String,
    pub query: String,
}

#[get("/search?<query>")]
fn search(repo: web::guard::Repository, profile: State<UserProfile>, query: String) -> Template {
    let mut question_dtos = repo.search_questions(query.clone())
                                .into_iter()
                                .map(|q| QuestionDTO::from(q))
                                .collect::<Vec<_>>();
    question_dtos.reverse();
    let context = SearchDTO {
        profile: ProfileDTO {
            username: profile.clone().name,
            image_url: String::from("/static/image/profile.jpg")
        },
        search_results: question_dtos,
        site_url: format!("https://{}/", env::var("APPLICATION_DOMAIN").unwrap()),
        query: query,
    };
    Template::render("search", &context)
}

/* POST /questions */

#[derive(FromForm)]
struct PostQuestionForm {
    body: String
}

#[derive(Serialize, Debug)]
struct PostQuestionFailedDTO {
    reason: String
}

#[post("/questions", data = "<params>")]
fn post_question(repo: web::guard::Repository, client_ip: web::guard::ClientIP, params: request::Form<PostQuestionForm>)
     -> Result<response::Redirect, Template> {
    match repo.store_question(params.body.clone(), client_ip.address()) {
        Ok(question) => {
            let question_id = question.id;
            notify::send_email(question);
            Ok(response::Redirect::to(format!("/question/{}/after_post", question_id)))
        },
        Err(err) => {
            match err {
                model::StoreQuestionError::BlankBody => {
                    let context = PostQuestionFailedDTO { reason: String::from("質問の内容が空です") };
                    Err(Template::render("question/post_failed", &context))
                }
            }
        }
    }
}

/* GET /question/after_post */

#[derive(Serialize, Debug)]
struct AfterPostQuestionDTO{
    pub question: QuestionDTO
}

#[get("/question/<id>/after_post")]
fn after_post_question(id: i32, repo: web::guard::Repository) -> Result<Template, response::Redirect> {
    if let Some(question) = repo.find_question(id) {
        let context = AfterPostQuestionDTO{
            question: QuestionDTO::from(question)
        };
        Ok(Template::render("question/after_post", &context))
    } else {
        Err(response::Redirect::to("/"))
    }
}

/* GET /question/<id> */

#[derive(Serialize, Debug)]
struct ShowQuestionDTO {
    pub question: QuestionDTO,
    pub next_question: Option<QuestionDTO>,
    pub prev_question: Option<QuestionDTO>,
}

#[get("/question/<id>")]
fn show_question(id: i32, repo: web::guard::Repository) -> Result<Template, status::NotFound<&'static str>> {
    if let Some(question) = repo.find_question(id) {
        if question.answered() {
            let next_question_opt = repo.find_next_question(question.id);
            let prev_question_opt = repo.find_prev_question(question.id);
            let context = ShowQuestionDTO {
                question: QuestionDTO::from(question),
                next_question: next_question_opt.map(|q| QuestionDTO::from(q)),
                prev_question: prev_question_opt.map(|q| QuestionDTO::from(q))
            };
            return Ok(Template::render("question/show", &context));
        }
    }

    return Err(status::NotFound("not found"));
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
                            .filter(|q| !q.hidden )
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
    tweet_sender: State<SyncSender<(i32, String, TextImage)>>,
    _auth: web::guard::BasicAuth
    ) -> response::Redirect {

    let answer_body = params.body.clone();
    if let Some(question) = repo.store_answer(id, answer_body.clone()) {
        let text_image = reing_text2image::TextImage::new(question.body, String::from("Reing"), (0x2c, 0x36, 0x5d));
        tweet_sender.send((id, answer_body, text_image)).unwrap();
    }
    response::Redirect::to("/admin")
}

/* POST /admin/question/<id>/hide */

#[post("/admin/question/<id>/hide")]
fn admin_hide_question(id: i32, repo: web::guard::Repository, _auth: web::guard::BasicAuth ) -> response::Redirect {
    let mut question = repo.find_question(id).unwrap();
    question.hidden = true;
    repo.update_question(question);

    response::Redirect::to("/admin")
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

#[catch(401)]
fn unauthorized(_req: &Request) -> RequireLogin {
    RequireLogin()
}

#[derive(Clone)]
struct UserProfile {
    pub name: String
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

    let (tweet_sender, tweet_receiver) = sync_channel(1000);

    thread::spawn(move || {
        loop {
            let (question_id, answer, question_image) = tweet_receiver.recv().unwrap();
            tweet::tweet_answer(question_id, answer, question_image);
            thread::sleep(time::Duration::from_secs(5 * 60));
        }
    });

    let user_profile = UserProfile {
        name: tweet::get_twitter_username()
    };

    rocket::ignite()
        .manage(pool)
        .manage(tweet_sender)
        .manage(user_profile)
        .mount("/", routes![
            index, index_with_page, files, post_question, after_post_question, show_question,
            admin_index, admin_post_answer, admin_show_question, admin_hide_question, search
        ])
        .register(catchers![unauthorized])
        .attach(Template::fairing())
        .launch();
}
