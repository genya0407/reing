use chrono::prelude::*;
use db;
use db::schema::{answers, questions};
use diesel;
use diesel::ExpressionMethods;
use diesel::QueryDsl;
use diesel::RunQueryDsl;
use diesel::SaveChangesDsl;
use r2d2;
use r2d2_diesel;
use std::ops::Deref;

type DieselConnection =
    r2d2::PooledConnection<r2d2_diesel::ConnectionManager<diesel::PgConnection>>;

#[derive(Debug, Clone)]
pub struct Question {
    pub id: i32,
    pub body: String,
    pub ip_address: String,
    pub created_at: DateTime<Local>,
    pub hidden: bool,
}

#[derive(Debug, Clone)]
pub struct Answer {
    pub id: i32,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub question: Question,
}

pub struct Repository {
    pooled_connection: DieselConnection,
}

#[derive(Debug)]
pub enum StoreQuestionError {
    BlankBody,
}

#[test]
fn pick_random_answer_test() {
    dotenv::dotenv().ok();
    let manager = r2d2_diesel::ConnectionManager::<diesel::PgConnection>::new(
        std::env::var("DATABASE_URL").unwrap(),
    );
    let pool = r2d2::Pool::builder().max_size(15).build(manager).unwrap();

    let mut picked_ids_count = std::collections::HashMap::<i32, i32>::new();
    for _ in 0..5000 {
        let repo = Repository::new(pool.get().unwrap());
        picked_ids_count
            .entry(repo.pick_random_answer().unwrap().id)
            .and_modify(|c| *c += 1)
            .or_insert(1);
    }

    let mut id_count = picked_ids_count.into_iter().collect::<Vec<(i32, i32)>>();
    id_count.sort_by_key(|(_id, count)| count.clone());
    for (id, count) in id_count {
        println!("Answer {}: {} times", id, count);
    }
}

impl Repository {
    pub fn new(pooled_connection: DieselConnection) -> Self {
        Self {
            pooled_connection: pooled_connection,
        }
    }

    fn conn(&self) -> &diesel::PgConnection {
        self.pooled_connection.deref()
    }

    pub fn store_question(
        &self,
        body: String,
        ip_address: String,
    ) -> Result<Question, StoreQuestionError> {
        if body.chars().all(|c| char::is_whitespace(c)) {
            Err(StoreQuestionError::BlankBody)
        } else {
            let new_question = db::NewQuestion {
                body: body,
                ip_address: ip_address,
            };

            let q: db::Question = diesel::insert_into(questions::table)
                .values(&new_question)
                .get_result(self.conn())
                .expect("Error saving new question");
            Ok(self.db2model_question(q))
        }
    }

    pub fn store_answer(&self, question_id: i32, body: String) -> Option<Answer> {
        self.find_question(question_id).map(|question| {
            // question_idのquestionが存在することを確認
            let new_answer = db::NewAnswer {
                question_id: question.id,
                body: body,
            };
            let a: db::Answer = diesel::insert_into(answers::table)
                .values(&new_answer)
                .get_result(self.conn())
                .expect("Error saving new answer");
            self.db2model_answer(a, question)
        })
    }

    pub fn answers(&self, offset: i64, count: i64) -> Vec<Answer> {
        let answers = answers::table
            .inner_join(questions::table)
            .order(answers::created_at.desc())
            .offset(offset)
            .limit(count)
            .load::<(db::Answer, db::Question)>(self.conn())
            .unwrap();
        answers
            .into_iter()
            .map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
            .collect()
    }

    pub fn not_answered_questions(&self) -> Vec<Question> {
        let qs = questions::table
            .left_join(answers::table)
            .filter(answers::id.is_null())
            .order(answers::created_at.desc())
            .load::<(db::Question, Option<db::Answer>)>(self.conn())
            .unwrap();
        qs.into_iter()
            .map(|(q, _)| self.db2model_question(q))
            .collect()
    }

    pub fn search_answers(&self, keywords_string: String) -> Vec<Answer> {
        use diesel::BoolExpressionMethods;
        use diesel::TextExpressionMethods;

        let mut query_of_keywords: Box<
            dyn diesel::BoxableExpression<_, _, SqlType = diesel::sql_types::Bool>,
        > = Box::new(diesel::dsl::not(questions::id.is_null()));
        for keyword in keywords_string.split(|c| c == ' ' || c == '　') {
            let keyword_query = format!("%{}%", keyword);
            let query_of_keyword = questions::body
                .like(keyword_query.clone())
                .or(answers::body.like(keyword_query.clone()));
            query_of_keywords = Box::new(query_of_keywords.and(query_of_keyword));
        }

        let answers = answers::table
            .inner_join(questions::table)
            .filter(query_of_keywords)
            .order(answers::created_at.desc())
            .load::<(db::Answer, db::Question)>(self.conn())
            .unwrap();
        answers
            .into_iter()
            .map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
            .collect()
    }

    pub fn find_question(&self, question_id: i32) -> Option<Question> {
        questions::table
            .filter(questions::id.eq(question_id))
            .limit(1)
            .load::<db::Question>(self.conn())
            .unwrap()
            .first()
            .cloned()
            .map(|q| self.db2model_question(q))
    }

    pub fn pick_random_answer(&self) -> Option<Answer> {
        use rand::Rng;

        let max_answer_id: i32 = match answers::table
            .select(diesel::dsl::max(answers::id))
            .first(self.conn())
        {
            Ok(Some(max_id)) => max_id,
            _ => return None,
        };
        let mut rng = rand::thread_rng();
        let random_id_lower_limit = rng.gen_range(0, max_answer_id);
        let answer = answers::table
            .inner_join(questions::table)
            .filter(answers::id.ge(random_id_lower_limit))
            .order(answers::id)
            .limit(1)
            .load::<(db::Answer, db::Question)>(self.conn())
            .unwrap()
            .first()
            .cloned();
        answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    }

    pub fn find_answer(&self, answer_id: i32) -> Option<Answer> {
        let answer = answers::table
            .inner_join(questions::table)
            .filter(answers::id.eq(answer_id))
            .limit(1)
            .load::<(db::Answer, db::Question)>(self.conn())
            .unwrap()
            .first()
            .cloned();
        answer.map(|(a, q)| self.db2model_answer(a, self.db2model_question(q)))
    }

    pub fn find_answer_by_question_id(&self, question_id: i32) -> Option<Answer> {
        let answer = answers::table
            .inner_join(questions::table)
            .filter(questions::id.eq(question_id))
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
            created_at: question.created_at.with_timezone(&Utc),
        };
        q.save_changes::<db::Question>(self.conn()).unwrap();
    }

    fn db2model_question(&self, q: db::Question) -> Question {
        Question {
            id: q.id,
            body: q.body,
            ip_address: q.ip_address,
            created_at: q.created_at.with_timezone(&Local),
            hidden: q.hidden,
        }
    }

    fn db2model_answer(&self, a: db::Answer, q: Question) -> Answer {
        Answer {
            id: a.id,
            body: a.body,
            created_at: a.created_at.with_timezone(&Local),
            question: q,
        }
    }
}
