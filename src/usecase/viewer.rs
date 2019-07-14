use super::repository::{
  UsesAnswerRepository,
  UsesQuestionRepository,
  AnswerRepository,
  QuestionRepository
};
use crate::entity::{Answer, Question};
use uuid::Uuid;
use chrono::Local;

pub trait SeeAllAnswersOutputPort {
  fn output(&self, answers: Vec<Answer>);
}

pub trait SeeAllAnswers: UsesAnswerRepository {
  fn execute(&self, output: Box<SeeAllAnswersOutputPort>) {
    let answers = self.answer_repository().find_all();
    output.output(answers)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[derive(Clone)]
  struct MockAnswerRepository {
    answers: Vec<Answer>
  }
  impl AnswerRepository for MockAnswerRepository {
    fn find_all(&self) -> Vec<Answer> {
      self.answers.clone()
    }

    fn store(&self, answer: Answer) {
      panic!("Not implemented")
    }

    fn find(&self, id: Uuid) -> Option<Answer> {
      panic!("Not implemented")
    }
  }

  struct SeeAllAnswersWithMock {
    answer_repository_mock: MockAnswerRepository
  }
  impl UsesAnswerRepository for SeeAllAnswersWithMock {
    fn answer_repository(&self) -> Box<AnswerRepository> {
      Box::new(self.answer_repository_mock.clone())
    }
  }
  impl SeeAllAnswers for SeeAllAnswersWithMock {}

  struct MockSeeAllAnswersOutputPort {
    expected_results: Vec<Answer>
  }
  impl SeeAllAnswersOutputPort for MockSeeAllAnswersOutputPort {
    fn output(&self, answers: Vec<Answer>) {
      assert_eq!(self.expected_results, answers)
    }
  }

  #[test]
  fn test_SeeAllAnswersImpl() {
    let question = Question {
      id: Uuid::new_v4(),
      body: String::from("aaa"),
      ip_address: String::from("0.0.0.0"),
      hidden: false,
      created_at: Local::now(),
    };
    let answers = vec![
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
    ];
    let mar = MockAnswerRepository{ answers: answers.clone() };
    let sawm = SeeAllAnswersWithMock { answer_repository_mock: mar };
    let mo = MockSeeAllAnswersOutputPort { expected_results: answers.clone() };
    sawm.execute(Box::new(mo));
  }
}