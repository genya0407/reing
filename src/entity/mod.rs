use chrono::prelude::*;
use uuid::Uuid;
use sha3::Sha3_256;
use sha3::Digest;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Validation<Reason: Eq> {
  Valid,
  Invalid(Reason)
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum QuestionInvalidReason {
  BlankBody
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum AnswerInvalidReason {
  BlankBody
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Question {
    pub id: Uuid,
    pub answerer_id: Uuid,
    pub body: String,
    pub ip_address: String,
    pub created_at: DateTime<Local>,
    pub hidden: bool
}

impl Question {
  pub fn new(answerer_id: Uuid, body: String, ip_address: String) -> Self {
    Self {
      id: Uuid::new_v4(),
      body: body,
      answerer_id: answerer_id,
      ip_address: ip_address,
      created_at: Local::now(),
      hidden: false,
    }
  }

  pub fn validate(&self) -> Validation<QuestionInvalidReason> {
    if self.body.trim().is_empty() {
      Validation::Invalid(QuestionInvalidReason::BlankBody)
    } else {
      Validation::Valid
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Answer {
    pub id: Uuid,
    pub body: String,
    pub created_at: DateTime<Local>,
    pub question: Question
}

impl Answer {
  pub fn validate(&self) -> Validation<AnswerInvalidReason> {
    if self.body.trim().is_empty() {
      Validation::Invalid(AnswerInvalidReason::BlankBody)
    } else {
      Validation::Valid
    }
  }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Answerer {
  pub id: Uuid,
  pub email: String,
  pub password_encrypted: String,
}

impl Answerer {
  pub fn new(email: String, password: String) -> Self {
    Self {
      id: Uuid::new_v4(),
      email: email,
      password_encrypted: bcrypt::hash(password, bcrypt::DEFAULT_COST).expect("hash generation failed")
    }
  }

  pub fn authenticate(&self, password: String) -> bool {
    bcrypt::verify(password, &self.password_encrypted).expect("password verification failed")
  }
}

#[test]
fn test_authentication() {
  let answerer = Answerer::new(String::from("example@example.com"), String::from("very very secure password"));
  assert!(answerer.authenticate(String::from("very very secure password")));
  assert!(!answerer.authenticate(String::from("vary very secure password")));
}