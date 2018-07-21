use chrono::prelude::*;
use r2d2;
use r2d2_diesel;
use diesel;
use db;
use std::ops::Deref;
use std::collections::HashMap;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::SaveChangesDsl;
use diesel::ExpressionMethods;
use db::schema::{questions, answers};

type DieselConnection = r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Clone)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub created_at: DateTime<Local>,
    pub hidden: bool,
    pub answers: Vec<Answer>,
}

impl Question {
    pub fn answered(&self) -> bool {
        return !self.answers.is_empty()
    }
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Local>
}

pub struct Repository {
    pooled_connection: DieselConnection
}

pub enum StoreQuestionError {
    BlankBody
}

impl Repository {
    pub fn new(pooled_connection: DieselConnection) -> Self {
        Self { pooled_connection: pooled_connection }
    }

    fn conn(&self) -> &diesel::PgConnection {
        self.pooled_connection.deref()
    }

    pub fn store_question(&self, body: String, ip_address: String) -> Result<Question, StoreQuestionError> {
        if body.chars().all(|c| char::is_whitespace(c)) {
            Err(StoreQuestionError::BlankBody)
        } else {
            let new_question = db::NewQuestion { body: body, ip_address: ip_address };

            let q: db::Question = diesel::insert_into(questions::table)
                    .values(&new_question)
                    .get_result(self.conn())
                    .expect("Error saving new question");
            Ok(self.qas2questions(vec![(q, None)]).into_iter().next().unwrap())
        }
    }

    pub fn store_answer(&self, id: i32, body: String) -> Option<Question> {
        self.find_question(id).map(|mut question| {
            let new_answer = db::NewAnswer {
                question_id: question.id,
                body: body
            };
            let a: db::Answer = diesel::insert_into(answers::table)
                .values(&new_answer)
                .get_result(self.conn())
                .expect("Error saving new answer");
            let answer = self.row2answer(a);
            question.answers.push(answer);
            question
        })
    }

    pub fn answered_questions(&self) -> Vec<Question> {
        let qas = questions::table
                .left_join(answers::table)
                .filter(answers::id.is_not_null())
                .load::<(db::Question, Option<db::Answer>)>(self.conn())
                .unwrap();
        self.qas2questions(qas)
    }

    pub fn not_answered_questions(&self) -> Vec<Question> {
        let qas = questions::table
                .left_join(answers::table)
                .filter(answers::id.is_null())
                .load::<(db::Question, Option<db::Answer>)>(self.conn())
                .unwrap();
        self.qas2questions(qas)                
    }

    pub fn find_question(&self, id: i32) -> Option<Question> {
        let qas = questions::table
                .left_join(answers::table)
                .filter(questions::id.eq(id))
                .limit(1)
                .load::<(db::Question, Option<db::Answer>)>(self.conn())
                .unwrap();
        self.qas2questions(qas).into_iter().next()
    }

    pub fn find_next_question(&self, id: i32) -> Option<Question> {
        let qas = questions::table
                .left_join(answers::table)
                .filter(questions::id.gt(id))
                .filter(answers::id.is_not_null())
                .order(questions::id.asc())
                .limit(1)
                .load::<(db::Question, Option<db::Answer>)>(self.conn())
                .unwrap();
        self.qas2questions(qas).into_iter().next()
    }

    pub fn find_prev_question(&self, id: i32) -> Option<Question> {
        let qas = questions::table
                .left_join(answers::table)
                .filter(questions::id.lt(id))
                .filter(answers::id.is_not_null())
                .order(questions::id.desc())
                .limit(1)
                .load::<(db::Question, Option<db::Answer>)>(self.conn())
                .unwrap();
        self.qas2questions(qas).into_iter().next()
    }

    // this method doesn't update answers.
    pub fn update_question(&self, question: Question) {
        let q = db::QuestionForm {
            id: question.id,
            body: question.body,
            ip_address: question.ip_address,
            hidden: question.hidden,
            created_at: question.created_at.with_timezone(&Utc)
        };
        q.save_changes::<db::Question>(self.conn()).unwrap();
    }

    fn qas2questions(&self, qas: Vec<(db::Question, Option<db::Answer>)>) -> Vec<Question> {
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

        let mut result = q_map.into_iter().map(|(_,v)| v).collect::<Vec<Question>>();
        result.sort_by_key(|q| q.created_at);
        return result;
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
