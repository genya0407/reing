use uuid::Uuid;
use chrono::{Local, DateTime};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswererAuthenticationFailedDTO {
  pub answerer_id: String,
  pub password: String
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuestionDTO {
  pub question_id: Uuid,
  pub question_body: String,
  pub questioned_at: DateTime<Local>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SeeAllNotAnsweredQuestionsError {
  AuthenticationFailed
}

pub trait Usecase {
  fn execute(&self, iport: Box<InputPort<AnswererAuthDTO>>, oport: Box<OutputPort<Result<Vec<QuestionDTO>, SeeAllNotAnsweredQuestionError>>>);
}