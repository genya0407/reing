use uuid::Uuid;
use chrono::{Local, DateTime};
use crate::usecase::repository::QuestionRepository;
use crate::usecase::{InputPort, OutputPort};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswererAuthenticationDTO {
  pub answerer_id: Uuid,
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
  fn execute(&self, iport: Box<InputPort<AnswererAuthenticationDTO>>, oport: Box<OutputPort<Result<Vec<QuestionDTO>, SeeAllNotAnsweredQuestionsError>>>);
}

mod implementation {
  use crate::usecase::{InputPort, OutputPort};
  use crate::usecase::repository::*;
  use super::*;

  struct Usecase {
    question_repo: Box<QuestionRepository>,
    answerer_repo: Box<AnswererRepository>,
  }

  impl super::Usecase for Usecase {
    fn execute(&self, iport: Box<InputPort<AnswererAuthenticationDTO>>, oport: Box<OutputPort<Result<Vec<QuestionDTO>, SeeAllNotAnsweredQuestionsError>>>) {
      let auth = iport.input();
      let answerer_opt = self.answerer_repo.find(auth.answerer_id);
      let output = match answerer_opt.map(|a| a.authenticate(auth.password.clone())) {
        Some(true) => Ok(
          self.question_repo
              .find_all_not_answered_yet_of(auth.answerer_id)
              .into_iter()
              .map(|q|
                QuestionDTO {
                  question_id: q.id,
                  question_body: q.body,
                  questioned_at: q.created_at
                }
              )
              .collect()
        ),
        _ => Err(SeeAllNotAnsweredQuestionsError::AuthenticationFailed),
      };
      oport.output(output);
    }
  }
}