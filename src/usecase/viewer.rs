use super::repository::AnswerRepository;
use crate::entity::{Answer, Question};
use uuid::Uuid;
use chrono::{Local, DateTime};
use std::collections::HashMap;
use crate::mock::repository::MockAnswerRepository;
use std::sync::{Mutex, Arc};

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

pub mod see_all_answers {
  use super::*;

  pub trait Usecase {
    fn execute(&self, output: Box<OutputPort>);
  }

  pub trait OutputPort {
    fn output(&self, answers: Vec<AnswerDTO>);
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
      fn execute(&self, output: Box<OutputPort>) {
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

    impl OutputPort for MockOutputPort {
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
}

pub mod see_answer_detail {
  use super::*;

  pub trait OutputPort {
    fn output(&self, answer: Option<AnswerDTO>);
  }

  pub trait InputPort {
    fn input(&self) -> Uuid;
  }

  pub trait Usecase {
    fn execute(&self, iport: Box<InputPort>, oport: Box<OutputPort>);
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
      fn execute(&self, iport: Box<InputPort>, oport: Box<OutputPort>) {
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

    impl OutputPort for MockOutputPort {
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

    impl InputPort for MockInputPort {
      fn input(&self) -> Uuid {
        self.data.clone()
      }
    }
  }

  #[cfg(test)]
  mod tests {
    use super::*;
    use super::mock::*;

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
}
