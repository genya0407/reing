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
use std::collections::HashMap;
use std::env;
use std::path::{Path, PathBuf};
use rocket::http::{Header, Status};
use rocket::request;
use rocket::response;
use rocket::response::status;
use rocket::Request;
use rocket_contrib::templates::Template;
use rocket_contrib::json::Json;
use chrono::prelude::*;
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
    pub question: QuestionDTO,
    pub created_at: DateTime<Local>,
    pub created_at_recognizable: String,
}

impl AnswerDTO {
    fn from(a: model::Answer) -> Self {
        Self {
            id: a.id, body: a.body, created_at: a.created_at,
            created_at_recognizable: utils::recognizable_datetime(a.created_at),
            question: QuestionDTO::from(a.question)
        }
    }
}

#[derive(Serialize, Debug)]
struct QuestionDTO {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub created_at_recognizable: String,
}

impl QuestionDTO {
    fn from(q: model::Question) -> Self {
        Self {
            id: q.id, body: q.body, created_at: q.created_at,
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
    pub answers: Vec<AnswerDTO>,
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

const ANSWER_COUNT_PER_PAGE : i64 = 30;
#[get("/")]
fn index(repo: web::guard::Repository, profile: State<UserProfile>) -> Template {
    let page = 0;
    index_with_page(repo, profile, page)
}

#[get("/page/<page>")]
fn index_with_page(repo: web::guard::Repository, profile: State<UserProfile>, page: i64) -> Template {
    let offset = page * ANSWER_COUNT_PER_PAGE;
    let answer_dtos = repo.answers(offset, ANSWER_COUNT_PER_PAGE)
                                .into_iter()
                                .map(|a| AnswerDTO::from(a))
                                .collect::<Vec<_>>();
    let (next_page, prev_page) = next_prev_page(page);
    let context = IndexDTO {
        profile: ProfileDTO {
            username: profile.clone().name,
            image_url: String::from("/static/image/profile.jpg")
        },
        answers: answer_dtos,
        site_url: format!("https://{}/", env::var("APPLICATION_DOMAIN").unwrap()),
        prev_page: prev_page,
        next_page: next_page,
    };
    Template::render("index", &context)
}

#[derive(Serialize, Debug)]
struct SearchDTO {
    pub profile: ProfileDTO,
    pub search_results: Vec<AnswerDTO>,
    pub site_url: String,
    pub query: String,
}

#[get("/search?<query>")]
fn search(repo: web::guard::Repository, profile: State<UserProfile>, query: String) -> Template {
    let answer_dtos = repo.search_answers(query.clone())
                                .into_iter()
                                .map(|a| AnswerDTO::from(a))
                                .collect::<Vec<_>>();
    let context = SearchDTO {
        profile: ProfileDTO {
            username: profile.clone().name,
            image_url: String::from("/static/image/profile.jpg")
        },
        search_results: answer_dtos,
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

#[get("/question/<question_id>/after_post")]
fn after_post_question(question_id: i32, repo: web::guard::Repository) -> Result<Template, response::Redirect> {
    if let Some(question) = repo.find_question(question_id) {
        let context = AfterPostQuestionDTO{
            question: QuestionDTO::from(question)
        };
        Ok(Template::render("question/after_post", &context))
    } else {
        Err(response::Redirect::to("/"))
    }
}

/* GET /answer/<question_id> */

#[derive(Serialize, Debug)]
struct ShowAnswerDTO {
    pub answer: AnswerDTO,
    pub next_answer: Option<AnswerDTO>,
    pub prev_answer: Option<AnswerDTO>,
}

#[get("/question/<question_id>")]
fn show_question(question_id: i32, repo: web::guard::Repository) -> Result<response::Redirect, status::NotFound<&'static str>> {
    match repo.find_answer_by_question_id(question_id) {
        Some(answer) => Ok(response::Redirect::to(format!("/answer/{}", answer.id))),
        None => Err(status::NotFound("not found"))
    }
}

#[get("/answer/<_answer_id>")]
fn show_answer(_answer_id: i32, app_env: State<AppEnvironment>) -> Template {
    let mut context: HashMap<String, bool> = HashMap::new();
    context.insert(String::from("is_production"), app_env.is_production);
    return Template::render("answer/show", &context);
}

#[get("/api/answer/<answer_id>")]
fn show_answer_json(answer_id: i32, repo: web::guard::Repository) -> Result<Json<ShowAnswerDTO>, status::NotFound<&'static str>> {
    if let Some(answer) = repo.find_answer(answer_id) {
        let next_answer_opt = repo.find_next_answer(answer.created_at);
        let prev_answer_opt = repo.find_prev_answer(answer.created_at);
        let context = ShowAnswerDTO {
            answer: AnswerDTO::from(answer),
            next_answer: next_answer_opt.map(|a| AnswerDTO::from(a)),
            prev_answer: prev_answer_opt.map(|a| AnswerDTO::from(a))
        };
        return Ok(Json(context));
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

/* GET /admin/question/<question_id> */

#[get("/admin/question/<question_id>")]
fn admin_show_question(question_id: i32, repo: web::guard::Repository, _auth: web::guard::BasicAuth) -> Template {
    let question = repo.find_question(question_id).unwrap();
    let context = QuestionDTO::from(question);
    Template::render("admin/questions/show", &context)
}

/* POST /question/<question_id>/answer */

#[derive(FromForm)]
struct PostAnswerForm {
    body: String
}

#[post("/admin/question/<question_id>/answer", data = "<params>")]
fn admin_post_answer(
    question_id: i32, repo: web::guard::Repository,
    params: request::Form<PostAnswerForm>,
    tweet_sender: State<SyncSender<model::Answer>>,
    _auth: web::guard::BasicAuth
    ) -> response::Redirect {

    let answer_body = params.body.clone();
    if let Some(answer) = repo.store_answer(question_id, answer_body.clone()) {
        tweet_sender.send(answer).unwrap();
    }
    response::Redirect::to("/admin")
}

/* POST /admin/question/<question_id>/hide */

#[post("/admin/question/<question_id>/hide")]
fn admin_hide_question(question_id: i32, repo: web::guard::Repository, _auth: web::guard::BasicAuth ) -> response::Redirect {
    let mut question = repo.find_question(question_id).unwrap();
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

#[derive(Clone)]
struct AppEnvironment {
    pub is_production: bool
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
            let answer = tweet_receiver.recv().unwrap();
            tweet::tweet_answer(answer);
            thread::sleep(time::Duration::from_secs(5 * 60));
        }
    });

    let user_profile = UserProfile {
        name: tweet::get_twitter_username()
    };

    let app_env = AppEnvironment {
        is_production: env::var("MODE").map(|mode| mode == "production").unwrap_or(false)
    };

    rocket::ignite()
        .manage(pool)
        .manage(tweet_sender)
        .manage(user_profile)
        .manage(app_env)
        .mount("/", routes![
            index, index_with_page, files, post_question, after_post_question, show_answer,
            admin_index, admin_post_answer, admin_show_question, admin_hide_question, search,
            show_question, show_answer_json
        ])
        .register(catchers![unauthorized])
        .attach(Template::fairing())
        .launch();
}
