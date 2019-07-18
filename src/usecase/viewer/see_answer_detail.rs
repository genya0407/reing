use crate::usecase::{InputPort, OutputPort};
use crate::usecase::repository::AnswerRepository;
use crate::usecase::viewer::{AnswerDTO, entity2dto};
use crate::entity::{Answer, Question};
use uuid::Uuid;
use std::sync::{Mutex, Arc};
use chrono::Local;

pub trait Usecase {
  fn execute(&self, iport: Box<InputPort<Uuid>>, oport: Box<OutputPort<Option<AnswerDTO>>>);
}

pub fn new(answer_repository: Box<AnswerRepository>) -> Box<Usecase> {
  Box::new(
    implement::Usecase{
      answer_repository: answer_repository
    }
  )
}

mod implement {
  use super::*;

  pub struct Usecase {
    pub answer_repository: Box<AnswerRepository>
  }

  impl super::Usecase for Usecase {
    fn execute(&self, iport: Box<InputPort<Uuid>>, oport: Box<OutputPort<Option<AnswerDTO>>>) {
      let answer_id = iport.input();
      let answer = self.answer_repository.find(answer_id).map(entity2dto);
      oport.output(answer)
    }
  }
}

mod mock {
  use super::*;

  #[derive(Clone)]
  pub struct MockOutputPort {
    pub result: Arc<Mutex<Option<AnswerDTO>>>
  }

  impl MockOutputPort {
    pub fn new() -> Box<MockOutputPort> {
      Box::new(Self { result: Arc::new(Mutex::new(None)) })
    }
  }

  impl OutputPort<Option<AnswerDTO>> for MockOutputPort {
    fn output(&self, answer: Option<AnswerDTO>) {
      let mut data = self.result.lock().unwrap();
      *data = answer;
    }
  }

  #[derive(Clone)]
  pub struct MockInputPort {
    pub data: Uuid
  }

  impl MockInputPort {
    pub fn new(id: Uuid) -> Box<MockInputPort> {
      Box::new(Self { data: id })
    }
  }

  impl InputPort<Uuid> for MockInputPort {
    fn input(&self) -> Uuid {
      self.data.clone()
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use super::mock::*;
  use crate::usecase::repository::mock::answer_respository::MockAnswerRepository;

  #[test]
  fn test_mock_output_port() {
    let mop = MockOutputPort::new();
    let answer_dto = AnswerDTO {
      answer_id: Uuid::new_v4(),
      answer_body: "answer1".to_string(),
      answered_at: Local::now(),
      question_id: Uuid::new_v4(),
      question_body: String::from("aaa"),
      questioned_at: Local::now(),
    };
    mop.output(Some(answer_dto.clone()));

    assert_eq!(*mop.result.lock().unwrap(), Some(answer_dto));
  }


  #[test]
  fn test_mock_input_port() {
    let id = Uuid::new_v4();
    let mip = MockInputPort::new(id);
    assert_eq!(mip.input(), id)
  }

  #[test]
  fn test_usecase() {
    use crate::usecase::repository::mock::answer_respository::MockAnswerRepository;

    let answer = Answer {
      id: Uuid::new_v4(),
      body: "answer1".to_string(),
      created_at: Local::now(),
      question: Question {
        id: Uuid::new_v4(),
        body: String::from("aaa"),
        ip_address: String::from("0.0.0.0"),
        hidden: false,
        created_at: Local::now(),
      }
    };
    let repo = MockAnswerRepository::new();
    repo.store(answer.clone());
    let usecase = new(repo);
    let iport = mock::MockInputPort::new(answer.id.clone());
    let oport = mock::MockOutputPort::new();
    usecase.execute(iport, oport.clone());

    let answer_dto_opt = oport.result.lock().unwrap();
    assert_eq!(answer_dto_opt.clone().map(|a| a.answer_id), Some(answer.id));
  }
}
