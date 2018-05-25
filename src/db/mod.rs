pub mod schema;

use self::schema::{questions, answers};
use chrono::prelude::*;

#[derive(Insertable)]
#[table_name="questions"]
pub struct NewQuestion {
    pub ip_address: String,
    pub body: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub hidden: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(AsChangeset, Identifiable)]
#[table_name = "questions"]
pub struct QuestionForm {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub hidden: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Insertable)]
#[table_name="answers"]
pub struct NewAnswer {
    pub question_id: i32,
    pub body: String,
}

#[derive(Queryable, Debug, Clone)]
pub struct Answer {
    pub id: i32,
    pub question_id: i32,
    pub body: String,
    pub created_at: DateTime<Utc>,
}