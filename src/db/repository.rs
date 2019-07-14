use r2d2;
use r2d2_diesel;
use diesel;
use crate::db;
use std::ops::Deref;
use diesel::RunQueryDsl;
use diesel::QueryDsl;
use diesel::SaveChangesDsl;
use diesel::ExpressionMethods;
use crate::db::schema::{questions, answers};
use crate::usecase::repository::{AnswerRepository, QuestionRepository, StoreAnswerError, QuestionAnswerError}
use log::warn;
use uuid::Uuid;
use crate::entity::{Answer, Question}

type DieselConnection = r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::PgConnection>>;

pub struct DieselRepository {
    pooled_connection: DieselConnection
}

impl DieselRepository {
    pub fn new(pooled_connection: DieselConnection) -> Self {
        Self { pooled_connection: pooled_connection }
    }

    fn conn(&self) -> &diesel::PgConnection {
        self.pooled_connection.deref()
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

    fn db2model_answer(&self, a: db::Answer, q: db::Question) -> Answer {
        Answer {
            id: a.id,
            body: a.body,
            created_at: a.created_at.with_timezone(&Local),
            question: self.db2model_question(q)
        }
    }
}

impl DieselRepository for AnswerRepository {
    pub fn store(&self, answer: Answer) {
        let new_answer = db::NewAnswer {
            question_id: question.id,
            body: answer.body
        };
        let a: db::Answer = diesel::insert_into(answers::table)
            .values(&new_answer)
            .get_result(self.conn())
            .map_err(|e| warn!("{}", e))
            .expect("Error saving new answer");
    }

    pub fn find(&self, id: Uuid) -> Option<Answer> {
        answers::table
            .filter(answers::id.eq(id))
            .limit(1)
            .load::<(db::Question, db::Answer)>(self.conn())
            .expect("failed fetch row from database")
            .first()
            .cloned()
            .map(|q, a| self.db2model_answer(a, q))
    }

    pub fn find_all(&self) -> Vec<Answer> {
        answers::table
            .filter(answers::id.eq(id))
            .load::<(db::Question, db::Answer)>(self.conn())
            .expect("failed fetch rows from database")
            .map(|q, a| self.db2model_answer(a, q))
    }
}

impl DieselRepository for QuestionAnswer {
    pub fn store(&self, question: Question) {
        let new_question = db::NewQuestion { body: question.body, ip_address: question.ip_address };

        let q: db::Question = diesel::insert_into(questions::table)
                .values(&new_question)
                .get_result(self.conn())
                .expect("Error saving new question");
    }

    pub fn find(&self, id: Uuid) -> Option<Answer> {

    }

    pub fn find_all_not_answered_yet(&self) -> Vec<Answer> {

    }
}
    // pub fn store_answer(&self, question_id: i32, body: String) -> Option<Answer> {
    //     self.find_question(question_id).map(|question| { // question_idのquestionが存在することを確認
    //         let new_answer = db::NewAnswer {
    //             question_id: question.id,
    //             body: body
    //         };
    //         let a: db::Answer = diesel::insert_into(answers::table)
    //             .values(&new_answer)
    //             .get_result(self.conn())
    //             .expect("Error saving new answer");
            
    //         self.db2model_answer(a, question)
    //     })
    // }

    // pub fn answers(&self, offset: i64, count: i64) -> Vec<Answer> {
    //     let answers = answers::table
    //             .inner_join(questions::table)
    //             .order(answers::created_at.desc())
    //             .offset(offset)
    //             .limit(count)
    //             .load::<(db::Answer, db::Question)>(self.conn())
    //             .unwrap();
    //     answers.into_iter().map(|(a, q)| self.db2model_answer(a, self.db2model_question(q))).collect()
    // }

    // pub fn not_answered_questions(&self) -> Vec<Question> {
    //     let qs = questions::table
    //             .left_join(answers::table)
    //             .filter(answers::id.is_null())
    //             .order(answers::created_at.desc())
    //             .load::<(db::Question, Option<db::Answer>)>(self.conn())
    //             .unwrap();
    //     qs.into_iter().map(|(q, _)| self.db2model_question(q)).collect()
    // }

    // pub fn search_answers(&self, query: String) -> Vec<Answer> {
    //     use diesel::TextExpressionMethods;
    //     use diesel::BoolExpressionMethods;

    //     let answers = answers::table
    //             .inner_join(questions::table)
    //             .filter(questions::body.like(&format!("%{}%", query)).or(answers::body.like(&format!("%{}%", query))))
    //             .order(answers::created_at.desc())
    //             .load::<(db::Answer, db::Question)>(self.conn())
    //             .unwrap();
    //     answers.into_iter().map(|(a, q)| self.db2model_answer(a, self.db2model_question(q))).collect()
    // }

    // pub fn find_question(&self, question_id: i32) -> Option<Question> {
    //     questions::table
    //         .filter(questions::id.eq(question_id))
    //         .limit(1)
    //         .load::<db::Question>(self.conn())
    //         .unwrap()
    //         .first()
    //         .cloned()
    //         .map(|q| self.db2model_question(q))
    // }

    // pub fn find_answer(&self, answer_id: i32) -> Option<Answer> {
    //     let answer = answers::table
    //             .inner_join(questions::table)
    //             .filter(answers::id.eq(answer_id))
    //             .limit(1)
    //             .load::<(db::Answer, db::Question)>(self.conn())
    //             .unwrap()
    //             .first()
    //             .cloned();
    //     answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    // }

    // pub fn find_answer_by_question_id(&self, question_id: i32) -> Option<Answer> {
    //     let answer = answers::table
    //             .inner_join(questions::table)
    //             .filter(questions::id.eq(question_id))
    //             .limit(1)
    //             .load::<(db::Answer, db::Question)>(self.conn())
    //             .unwrap()
    //             .first()
    //             .cloned();
    //     answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    // }

    // pub fn find_next_answer(&self, after: DateTime<Local>) -> Option<Answer> {
    //     let answer = answers::table
    //             .inner_join(questions::table)
    //             .filter(answers::created_at.gt(after))
    //             .order(answers::created_at.asc())
    //             .limit(1)
    //             .load::<(db::Answer, db::Question)>(self.conn())
    //             .unwrap()
    //             .first()
    //             .cloned();
    //     answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    // }

    // pub fn find_prev_answer(&self, before: DateTime<Local>) -> Option<Answer> {
    //     let answer = answers::table
    //             .inner_join(questions::table)
    //             .filter(answers::created_at.lt(before))
    //             .order(answers::created_at.desc())
    //             .limit(1)
    //             .load::<(db::Answer, db::Question)>(self.conn())
    //             .unwrap()
    //             .first()
    //             .cloned();
    //     answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    // }

    // pub fn update_question(&self, question: Question) {
    //     let q = db::QuestionForm {
    //         id: question.id,
    //         body: question.body,
    //         ip_address: question.ip_address,
    //         hidden: question.hidden,
    //         created_at: question.created_at.with_timezone(&Utc)
    //     };
    //     q.save_changes::<db::Question>(self.conn()).unwrap();
    // }
