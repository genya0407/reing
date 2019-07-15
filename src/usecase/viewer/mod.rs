use chrono::{Local, DateTime};
use crate::entity::Answer;
use uuid::Uuid;

fn entity2dto(a: Answer) -> AnswerDTO {
  AnswerDTO {
    answer_id: a.id,
    answer_body: a.body,
    answered_at: a.created_at,
    question_id: a.question.id,
    question_body: a.question.body,
    questioned_at: a.question.created_at,
  }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct AnswerDTO {
  answer_id: Uuid,
  answer_body: String,
  answered_at: DateTime<Local>,
  question_id: Uuid,
  question_body: String,
  questioned_at: DateTime<Local>,
}

pub mod see_all_answers;
pub mod see_answer_detail;