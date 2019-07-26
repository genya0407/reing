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
    pub body: String,
    pub ip_address: String,
    pub created_at: DateTime<Local>,
    pub hidden: bool
}

impl Question {
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
    pub answerer_id: Uuid,
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
  pub salt: Vec<u8>,
  pub password_encrypted: Vec<u8>,
}

impl Answerer {
  pub fn encrypt(&self, plain: Vec<u8>) -> Vec<u8> {
    let mut digest = plain.clone();
    digest.extend(self.salt.clone().into_iter());

    for _ in 0..1024 {
      digest = Sha3_256::digest(&digest).as_slice().to_vec()
    }

    digest
  }

  pub fn authenticate(&self, password: String) -> bool {
    self.password_encrypted == self.encrypt(password.as_bytes().to_vec()).as_slice()
  }
}