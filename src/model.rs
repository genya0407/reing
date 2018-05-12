use chrono::prelude::*;
use r2d2;
use r2d2_diesel;
use diesel;
use db;
use std::ops::Deref;
use diesel::RunQueryDsl;

type DieselConnection = r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub created_at: DateTime<Local>,
    pub hidden: bool,
    pub answers: Vec<Answer>,
}

#[derive(Debug)]
pub struct Answer {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Local>
}

pub struct Repository {
    pooled_connection: DieselConnection
}

impl Repository {
    pub fn new(pooled_connection: DieselConnection) -> Self {
        Self { pooled_connection: pooled_connection }
    }

    fn conn(&self) -> &diesel::PgConnection {
        self.pooled_connection.deref()
    }

    pub fn store_question(&self, body: String, ip_address: String) -> Question {
        let new_question = db::NewQuestion { body: body, ip_address: ip_address };

        let question = {
            use db::schema::questions::dsl::*;
            let q: db::Question = diesel::insert_into(questions)
                    .values(&new_question)
                    .get_result(self.conn())
                    .expect("Error saving new post");
            Question {
                id: q.id,
                body: q.body,
                ip_address: q.ip_address,
                created_at: q.created_at.with_timezone(&Local),
                hidden: q.hidden,
                answers: vec![]
            }
        };
        return question;
    }

    pub fn all_questions(&self) -> Vec<Question> {
        use db::schema::questions;

        let qs = questions::table.load::<db::Question>(self.conn()).unwrap();
        qs.into_iter().map(|q| {
            Question {
                id: q.id,
                body: q.body,
                ip_address: q.ip_address,
                created_at: q.created_at.with_timezone(&Local),
                hidden: q.hidden,
                answers: vec![]
            }
        }).collect::<Vec<Question>>()
    }

    pub fn find_question(&self, id: i32) -> Option<Question> {
        Some(Question {
            id: 0, body: "".to_string(), ip_address: "".to_string(), created_at: Local::now(),
            hidden: false, answers: vec![]
        })
    }

    pub fn hide_question(&self, _id: i32) {
    }
}
