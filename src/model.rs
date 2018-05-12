use chrono::prelude::*;
use r2d2;
use r2d2_diesel;
use diesel;

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
    conn: DieselConnection
}

impl Repository {
    pub fn new(conn: DieselConnection) -> Self {
        Self { conn: conn }
    }

    pub fn store_question(&self, body: String, ip_address: String) -> Question {
        Question {
            id: 0, body: "".to_string(), ip_address: "".to_string(), created_at: Local::now(),
            hidden: false, answers: vec![]
        }
    }

    pub fn all_questions(&self) -> Vec<Question> {
        vec![]
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
