use super::repository::{
  AnswerRepository,
  QuestionRepository
};
use crate::entity::{Answer, Question};
use uuid::Uuid;
use chrono::Local;
use std::collections::HashMap;
use crate::mock;
use std::sync::Mutex;

pub mod see_all_answers {
  use super::*;

  pub trait Usecase {
    fn execute(&self, output: Box<OutputPort>);
  }

  pub trait OutputPort {
    fn output(&self, answers: Vec<Answer>);
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
        let answers = self.answer_repository.find_all();
        output.output(answers)
      }
    }
  }

  mod mock {
    use super::*;
    use std::sync::Mutex;

    struct MockOutputPort {
      result: Mutex<Vec<Answer>>
    }

    impl MockOutputPort {
      fn output(&self, answers: Vec<Answer>) {
        let mut data = self.result.lock().unwrap();
        *data = answers;
      }
    }

    #[test]
    fn test_mock_output_port() {
      let mop = MockOutputPort { result: Mutex::new(vec![]) };
      let question = Question {
        id: Uuid::new_v4(),
        body: String::from("aaa"),
        ip_address: String::from("0.0.0.0"),
        hidden: false,
        created_at: Local::now(),
      };
      mop.output(
        vec![
          Answer {
            id: Uuid::new_v4(),
            body: "answer1".to_string(),
            created_at: Local::now(),
            question: question.clone(),
          },
          Answer{
            id: Uuid::new_v4(),
            body: "answer2".to_string(),
            created_at: Local::now(),
            question: question.clone(),
          }
        ]
      );

      let output_answers = mop.result.lock().unwrap();
      assert_eq!(output_answers.len(), 2)
    }
  }

  #[cfg(test)]
  mod tests {

    #[test]
    fn test_usecase() {

    }
  }
}

mod see_answer_detail {
  use super::*;

  pub trait OutputPort {
    fn output(&self, answer: Option<Answer>);
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
        let answer = self.answer_repository.find(answer_id);
        oport.output(answer)
      }
    }
  }
}

// #[cfg(test)]
// mod tests {
//   mod see_all_answers {
//     use super::super::*;

//     struct SeeAllAnswersWithMock {
//       answer_repository_mock: MockAnswerRepository
//     }
//     impl see_all_answers

//     struct MockSeeAllAnswersOutputPort {
//       expected_results: Vec<Answer>
//     }
//     impl SeeAllAnswersOutputPort for MockSeeAllAnswersOutputPort {
//       fn output(&self, answers: Vec<Answer>) {
//         assert_eq!(self.expected_results, answers)
//       }
//     }

//     #[test]
//     fn test_SeeAllAnswersImpl() {
//       let question = Question {
//         id: Uuid::new_v4(),
//         body: String::from("aaa"),
//         ip_address: String::from("0.0.0.0"),
//         hidden: false,
//         created_at: Local::now(),
//       };
//       let answers = vec![
//         Answer {
//           id: Uuid::new_v4(),
//           body: "answer1".to_string(),
//           created_at: Local::now(),
//           question: question.clone(),
//         },
//         Answer{
//           id: Uuid::new_v4(),
//           body: "answer2".to_string(),
//           created_at: Local::now(),
//           question: question.clone(),
//         }
//       ];
//       let mar = MockAnswerRepository{ answers: answers.clone() };
//       let sawm = SeeAllAnswersWithMock { answer_repository_mock: mar };
//       let mo = MockSeeAllAnswersOutputPort { expected_results: answers.clone() };
//       sawm.execute(Box::new(mo));
//     }
//   }
// }