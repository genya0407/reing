use chrono::prelude::*;
use uuid::Uuid;

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

