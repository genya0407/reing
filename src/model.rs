use chrono::prelude::*;
use r2d2;
use r2d2_diesel;
use diesel;
use db;
use std::ops::Deref;
use std::collections::HashMap;
use diesel::RunQueryDsl;
use diesel::QueryDsl;

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
        use db::schema::{questions, answers};

        let qas = questions::table
                .left_join(answers::table)
                .load::<(db::Question, Option<db::Answer>)>(self.conn())
                .unwrap();

        let mut q_map: HashMap<i32, Question> = HashMap::new();
        for (q, a_opt) in qas {
            q_map.entry(q.id)
                .and_modify(|question| {
                    if let Some(a) = a_opt.clone() {
                        question.answers.push(self.row2answer(a))
                    }
                }).or_insert_with(|| {
                    let mut question = self.row2question(q);
                    if let Some(a) = a_opt {
                        question.answers.push(self.row2answer(a))
                    }
                    question
                });
        }

        q_map.into_iter().map(|(_,v)| v).collect::<Vec<Question>>()
    }

    pub fn find_question(&self, id: i32) -> Option<Question> {
        Some(Question {
            id: 0, body: "".to_string(), ip_address: "".to_string(), created_at: Local::now(),
            hidden: false, answers: vec![]
        })
    }

    fn row2question(&self, q: db::Question) -> Question {
        Question {
            id: q.id,
            body: q.body,
            ip_address: q.ip_address,
            created_at: q.created_at.with_timezone(&Local),
            hidden: q.hidden,
            answers: vec![]
        }
    }

    fn row2answer(&self, a: db::Answer) -> Answer {
        Answer {
            id: a.id,
            body: a.body,
            created_at: a.created_at.with_timezone(&Local)
        }
    }
}
