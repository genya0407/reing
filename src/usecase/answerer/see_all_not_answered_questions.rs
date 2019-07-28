use uuid::Uuid;
use chrono::{Local, DateTime};
use crate::usecase::repository::*;
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

pub fn new(question_repo: Box<QuestionRepository>, answerer_repo: Box<AnswererRepository>) -> Box<Usecase> {
  Box::new(implementation::Usecase::new(question_repo, answerer_repo))
}

mod implementation {
  use crate::usecase::{InputPort, OutputPort};
  use crate::usecase::repository::*;
  use super::*;

  pub struct Usecase {
    question_repo: Box<QuestionRepository>,
    answerer_repo: Box<AnswererRepository>,
  }

  impl Usecase {
    pub fn new(question_repo: Box<QuestionRepository>, answerer_repo: Box<AnswererRepository>) -> Self {
      Self {
        question_repo: question_repo,
        answerer_repo: answerer_repo,
      }
    }
  }

  impl super::Usecase for Usecase {
    fn execute(&self, iport: Box<InputPort<AnswererAuthenticationDTO>>, oport: Box<OutputPort<Result<Vec<QuestionDTO>, SeeAllNotAnsweredQuestionsError>>>) {
      let auth = iport.input();
      let answerer_opt = self.answerer_repo.find_by_id(auth.answerer_id);
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

#[cfg(tests)]
mod tests {
  use crate::entity::*;

  #[test]
  fn test_success() {
    let a1 = Answerer::new(String::from("example1@example.com"), String::from("very secure password"));
    let q1 =  Question::new(a1.id, String::from("body1"), String::from("0.0.0.0"));

    let iport = crate::usecase::MockInputPort { value: a1.id };
    let oport = crate::usecase::MockOutputPort::new();

    let answerer_repo = crate::usecase::repository::mock::answerer_respository::MockAnswererRepository::new();
    let question_repo = crate::usecase::repository::mock::question_repository::MockQuestionRepository::new();
    answerer_repo.store(a1);
    question_repo.store(q1.clone());

    let usecase = super::new(answerer_repo, question_repo);

    usecase.execute(iport, oport.clone());
    let output = oport.value.lock().unwrap();

    assert_eq!(output, Ok(vec![q1]));
  }

  #[test]
  fn test_wrong_password() {

  }

  #[test]
  fn test_not_exist_answerer() {

  }
}