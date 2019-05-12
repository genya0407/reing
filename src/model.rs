use chrono::prelude::*;
use r2d2;
use r2d2_diesel;
use diesel;
use db;
use std::ops::Deref;
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
    pub hidden: bool
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub question: Question
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
            Ok(self.db2model_question(q))
        }
    }

    pub fn store_answer(&self, id: i32, body: String) -> Option<Answer> {
        self.find_question(id).map(|mut question| {
            let new_answer = db::NewAnswer {
                question_id: question.id,
                body: body
            };
            let a: db::Answer = diesel::insert_into(answers::table)
                .values(&new_answer)
                .get_result(self.conn())
                .expect("Error saving new answer");
            
            self.db2model_answer(a, question)
        })
    }

    pub fn answers(&self, offset: i64, count: i64) -> Vec<Question> {
        let answers = answers::table
                .inner_join(questions::table)
                .order(answers::created_at.desc())
                .offset(offset)
                .limit(count)
                .load::<(db::Answer, db::Question)>(self.conn())
                .unwrap();
        answers.into_iter().map(|(a, q)| self.db2model_answer(a, self.db2model_question(q))).collect()
    }

    pub fn not_answered_questions(&self) -> Vec<Question> {
        let qs = questions::table
                .left_join(answers::table)
                .filter(answers::id.is_null())
                .order(answers::created_at.desc())
                .load::<(db::Question, Option<db::Answer>)>(self.conn())
                .unwrap();
        qs.into_iter().map(|(q, _)| self.db2model_question(q)).collect()
    }

    pub fn search_answers(&self, query: String) -> Vec<Answer> {
        use diesel::TextExpressionMethods;
        use diesel::BoolExpressionMethods;

        let answers = answers::table
                .inner_join(questions::table)
                .filter(questions::body.like(&format!("%{}%", query)).or(answers::body.like(&format!("%{}%", query))))
                .order(answers::created_at.desc())
                .load::<(db::Answer, db::Question)>(self.conn())
                .unwrap();
        answers.into_iter().map(|(a, q)| self.db2model_answer(a, self.db2model_question(q))).collect()
    }

    pub fn find_answer(&self, id: i32) -> Option<Answer> {
        let answer = answers::table
                .inner_join(questions::table)
                .filter(questions::id.eq(id))
                .limit(1)
                .load::<(db::Answer, db::Question)>(self.conn())
                .unwrap()
                .first()
                .cloned();
        answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    }

    pub fn find_next_answer(&self, after: DateTime<Local>) -> Option<Answer> {
        let answer = answers::table
                .inner_join(questions::table)
                .filter(answers::created_at.gt(after))
                .order(answers::created_at.asc())
                .limit(1)
                .load::<(db::Answer, db::Question)>(self.conn())
                .unwrap()
                .first()
                .cloned();
        answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    }

    pub fn find_prev_answer(&self, before: DateTime<Local>) -> Option<Answer> {
        let answer = answers::table
                .inner_join(questions::table)
                .filter(answers::created_at.lt(before))
                .order(answers::created_at.desc())
                .limit(1)
                .load::<(db::Answer, db::Question)>(self.conn())
                .unwrap()
                .first()
                .cloned();
        answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    }

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

    fn db2model_question(&self, q: db::Question) -> Question {
        Question {
            id: q.id,
            body: q.body,
            ip_address: q.ip_address,
            created_at: q.created_at.with_timezone(&Local),
            hidden: q.hidden
        }
    }

    fn db2model_answer(&self, a: db::Answer, q: Question) -> Answer {
        Answer {
            id: a.id,
            body: a.body,
            created_at: a.created_at.with_timezone(&Local),
            question: q
        }
    }
}
