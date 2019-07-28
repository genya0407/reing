use crate::entity::Question;
use crate::entity;
use crate::usecase::repository::QuestionRepository;
use crate::usecase::{InputPort, OutputPort};
use uuid::Uuid;
use chrono::{Local, DateTime};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct NewQuestionDTO {
  pub question_body: String,
  pub question_ip_address: String,
  pub answerer_id: Uuid,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct QuestionDTO {
  pub question_id: Uuid,
  pub question_body: String,
  pub questioned_at: DateTime<Local>,
}

pub fn new(question_repository: Box<QuestionRepository>) -> Box<Usecase> {
  Box::new(implement::Usecase { question_repository: question_repository })
}

fn model2dto(question: Question) -> QuestionDTO {
  QuestionDTO {
    question_id: question.id,
    question_body: question.body,
    questioned_at: question.created_at,
  }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PostQuestionError {
  BlankBody
}

trait Usecase {
  fn execute(&self, iport: Box<InputPort<NewQuestionDTO>>, oport: Box<OutputPort<Result<QuestionDTO, PostQuestionError>>>);
}

mod implement {
  use super::*;

  pub struct Usecase {
    pub question_repository: Box<QuestionRepository>
  }

  impl super::Usecase for Usecase {
    fn execute(&self, iport: Box<InputPort<NewQuestionDTO>>, oport: Box<OutputPort<Result<QuestionDTO, PostQuestionError>>>) {
      let new_question_dto = iport.input();
      let question = Question {
        id: Uuid::new_v4(),
        answerer_id: new_question_dto.answerer_id,
        body: new_question_dto.question_body,
        ip_address: new_question_dto.question_ip_address,
        created_at: Local::now(),
        hidden: false,
      };
      let result = match question.validate() {
        entity::Validation::Valid => {
          self.question_repository.store(question.clone());
          Ok(model2dto(question))
        },
        entity::Validation::Invalid(entity::QuestionInvalidReason::BlankBody) => {
          Err(PostQuestionError::BlankBody)
        }
      };
      oport.output(result);
    }
  }
}

mod mock {
  use super::*;
  use crate::usecase::{InputPort, OutputPort};
  use std::sync::{Mutex, Arc};

  pub struct MockInputPort {
    pub value: NewQuestionDTO
  }

  #[derive(Clone)]
  pub struct MockOutputPort {
    pub value: Arc<Mutex<Option<Result<QuestionDTO, PostQuestionError>>>>
  }

  impl InputPort<NewQuestionDTO> for MockInputPort {
    fn input(&self) -> NewQuestionDTO {
      return self.value.clone()
    }
  }

  impl OutputPort<Result<QuestionDTO, PostQuestionError>> for MockOutputPort {
    fn output(&self, output: Result<QuestionDTO, PostQuestionError>) {
      let mut value = self.value.lock().unwrap();
      *value = Some(output);
    }
  }

  impl MockOutputPort {
    pub fn new() -> Self {
      Self { value: Arc::new(Mutex::new(None)) }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::usecase::repository::mock::question_repository::MockQuestionRepository;

  #[test]
  fn test_valid_request() {
    let question_repository = Box::new(MockQuestionRepository::new());
    let usecase = new(question_repository);

    let iport = Box::new(
      mock::MockInputPort {
        value: NewQuestionDTO {
          question_body: String::from("Some body"),
          question_ip_address: String::from("10.0.0.1"),
          answerer_id: Uuid::new_v4(),
        }
      }
    );
    let oport = Box::new(mock::MockOutputPort::new());

    usecase.execute(iport, oport.clone());

    let lock = oport.value.lock();
    let result: Result<QuestionDTO, PostQuestionError> = lock.unwrap().clone().unwrap();
    assert_eq!(result.map(|q| q.question_body), Ok("Some body".to_string()));
  }

  #[test]
  fn test_blank_body_request() {
    let question_repository = Box::new(MockQuestionRepository::new());
    let usecase = new(question_repository);

    let iport = Box::new(
      mock::MockInputPort {
        value: NewQuestionDTO {
          question_body: String::from(""),
          question_ip_address: String::from("10.0.0.1"),
          answerer_id: Uuid::new_v4(),
        }
      }
    );
    let oport = Box::new(mock::MockOutputPort::new());

    usecase.execute(iport, oport.clone());

    let lock = oport.value.lock();
    let result: Result<QuestionDTO, PostQuestionError> = lock.unwrap().clone().unwrap();
    assert_eq!(result, Err(PostQuestionError::BlankBody));
  }
}