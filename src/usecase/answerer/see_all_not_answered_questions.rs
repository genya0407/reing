use uuid::Uuid;
use chrono::{Local, DateTime};
use crate::usecase::repository::QuestionRepository;
use crate::usecase::{InputPort, OutputPort};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AnswererAuthenticationDTO {
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
  fn execute(&self, iport: Box<InputPort<AnswererAuthenticationDTO>>, oport: Box<OutputPort<Result<Vec<QuestionDTO>, SeeAllNotAnsweredQuestionsError>>>);
}

// mod implementation {
//   use crate::usecase::{InputPort, OutputPort};

//   struct Usecase {
//     repo: Box<QuestionRepository>
//   }

//   impl super::Usecase for Usecase {
//     fn execute(&self, iport: Box<InputPort<AnswererAuthDTO>>, oport: Box<OutputPort<Result<Vec<QuestionDTO>, SeeAllNotAnsweredQuestionError>>>) {
//       let auth = iport.input();
//       self.repo.find
//     }
//   }
// }