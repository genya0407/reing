use crate::usecase::OutputPort;
use crate::usecase::repository::AnswerRepository;
use crate::usecase::viewer::{AnswerDTO, entity2dto};
use crate::entity::{Answer, Question};
use uuid::Uuid;
use std::sync::{Mutex, Arc};
use chrono::Local;
use std::collections::HashMap;

pub trait Usecase {
  fn execute(&self, output: Box<OutputPort<Vec<AnswerDTO>>>);
}

pub fn new(repo: Box<AnswerRepository>) -> Box<Usecase> {
  Box::new(
    implement::Usecase {
      answer_repository: repo
    }
  )
}

mod implement {
  use super::*;

  pub struct Usecase {
    pub answer_repository: Box<AnswerRepository>
  }

  impl super::Usecase for Usecase {
    fn execute(&self, output: Box<OutputPort<Vec<AnswerDTO>>>) {
      let answer_dtos = self
                          .answer_repository
                          .find_all()
                          .into_iter()
                          .map(entity2dto)
                          .collect();
      output.output(answer_dtos)
    }
  }
}

mod mock {
  use super::*;

  #[derive(Clone)]
  pub struct MockOutputPort {
    pub result: Arc<Mutex<Vec<AnswerDTO>>>
  }

  impl MockOutputPort {
    pub fn new() -> Box<MockOutputPort> {
      Box::new(Self { result: Arc::new(Mutex::new(vec![])) })
    }
  }

  impl OutputPort<Vec<AnswerDTO>> for MockOutputPort {
    fn output(&self, answers: Vec<AnswerDTO>) {
      let mut data = self.result.lock().unwrap();
      *data = answers;
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::mock::*;

  #[test]
  fn test_mock_output_port() {
    let mop = mock::MockOutputPort::new();
    mop.output(
      vec![
        AnswerDTO {
          answer_id: Uuid::new_v4(),
          answer_body: "answer1".to_string(),
          answered_at: Local::now(),
          question_id: Uuid::new_v4(),
          question_body: "question1".to_string(),
          questioned_at: Local::now(),
        },
        AnswerDTO {
          answer_id: Uuid::new_v4(),
          answer_body: "answer2".to_string(),
          answered_at: Local::now(),
          question_id: Uuid::new_v4(),
          question_body: "question2".to_string(),
          questioned_at: Local::now(),
        },
      ]
    );

    let output_answers = mop.result.lock().unwrap();
    assert_eq!(output_answers.len(), 2)
  }

  #[test]
  fn test_usecase() {
    use crate::mock::repository::MockAnswerRepository;

    let answer_id = Uuid::new_v4();
    let repo = MockAnswerRepository::new();
    repo.store(
      Answer {
        id: answer_id,
        body: "answer1".to_string(),
        created_at: Local::now(),
        question: Question {
          id: Uuid::new_v4(),
          body: String::from("aaa"),
          ip_address: String::from("0.0.0.0"),
          hidden: false,
          created_at: Local::now(),
        }
      }
    );
    let oport = MockOutputPort::new();
    let usecase = super::new(repo);

    usecase.execute(oport.clone());

    let result_answers = oport.result.lock().unwrap();
    assert_eq!(result_answers.first().unwrap().answer_id, answer_id)
  }
}
