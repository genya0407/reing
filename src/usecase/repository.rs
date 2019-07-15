use crate::entity::{Answer, Question};
use uuid::Uuid;

pub trait AnswerRepository {
  fn store(&self, answer: Answer);
  fn find(&self, id: Uuid) -> Option<Answer>;
  fn find_all(&self) -> Vec<Answer>;
}

pub trait QuestionRepository {
  fn store(&self, question: Question);
  fn find(&self, id: Uuid) -> Option<Question>;
  fn find_all_not_answered_yet(&self) -> Vec<Question>;
}