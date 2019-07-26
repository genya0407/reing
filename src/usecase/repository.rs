use crate::entity::{Answer, Question, Answerer};
use uuid::Uuid;

pub trait AnswerRepository {
  fn store(&self, answer: Answer);
  fn find(&self, id: Uuid) -> Option<Answer>;
  fn find_all_of(&self, answerer_id: Uuid) -> Vec<Answer>;
}

pub trait QuestionRepository {
  fn store(&self, question: Question);
  fn find(&self, id: Uuid) -> Option<Question>;
  fn find_all_not_answered_yet_of(&self, answerer_id: Uuid) -> Vec<Question>;
}

pub trait AnswererRepository {
  fn find(&self, id: Uuid) -> Option<Answerer>;
}

pub mod mock {
  use std::collections::HashMap;
  use uuid::Uuid;
  use std::sync::Mutex;
  use crate::entity::{Answer, Question, Answerer};
  use super::{QuestionRepository, AnswerRepository, AnswererRepository};

  pub mod question_repository {
    use super::*;

    pub struct MockQuestionRepository {
      pub questions: Mutex<HashMap<Uuid, Question>>
    }

    impl MockQuestionRepository {
      pub fn new() -> Self {
        Self {
          questions: Mutex::new(HashMap::new())
        }
      }
    }

    impl QuestionRepository for MockQuestionRepository {
      fn find(&self, id: Uuid) -> Option<Question> {
        self.questions.lock().unwrap().get(&id).cloned()
      }

      fn store(&self, question: Question) {
        let mut questions = self.questions.lock().unwrap();
        questions.insert(question.id, question);
      }

      fn find_all_not_answered_yet_of(&self, answerer_id: Uuid) -> Vec<Question> {
        panic!("not implemented")
      }
    }
  }

  pub mod answer_respository {
    use super::*;

    pub struct MockAnswerRepository {
      pub answers: Mutex<HashMap<Uuid, Answer>>
    }

    impl MockAnswerRepository {
      pub fn new() -> Box<AnswerRepository> {
        Box::new(
          Self {
            answers: Mutex::new(HashMap::new())
          }
        )
      }
    }

    impl AnswerRepository for MockAnswerRepository {
      fn find_all_of(&self, answerer_id: Uuid) -> Vec<Answer> {
        self.answers.lock().unwrap().values().map(|a| a.clone()).filter(|a| a.answerer_id == answerer_id).collect()
      }

      fn store(&self, answer: Answer) {
        let mut answers = self.answers.lock().unwrap();
        answers.insert(answer.id, answer);
      }

      fn find(&self, id: Uuid) -> Option<Answer> {
        self.answers.lock().unwrap().get(&id).cloned()
      }
    }
  }

  pub mod answerer_respository {
    use super::*;

    pub struct MockAnswererRepository {
      pub answerers: Mutex<HashMap<Uuid, Answerer>>
    }

    impl MockAnswererRepository {
      pub fn new() -> Box<AnswererRepository> {
        Box::new(
          Self {
            answerers: Mutex::new(HashMap::new())
          }
        )
      }
    }

    impl AnswererRepository for MockAnswererRepository {
      fn find(&self, id: Uuid) -> Option<Answerer> {
        self.answerers.lock().unwrap().get(&id).cloned()
      }
    }
  }
}