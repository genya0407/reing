use std::collections::HashMap;
use uuid::Uuid;
use crate::entity::Answer;
use std::sync::Mutex;
use crate::usecase::repository::AnswerRepository;

pub mod repository {
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
    fn find_all(&self) -> Vec<Answer> {
      self.answers.lock().unwrap().values().map(|a| a.clone()).collect()
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