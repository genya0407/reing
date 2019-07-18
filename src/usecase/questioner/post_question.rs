use crate::entity::Question;
use crate::entity;
use crate::usecase::repository::QuestionRepository;
use uuid::Uuid;
use chrono::{Local, DateTime};

pub struct NewQuestionDTO {
  pub question_body: String,
  pub ip_address: String,
  pub questioned_at: DateTime<Local>,
}

pub struct QuestionDTO {
  pub question_id: Uuid,
  pub question_body: String,
  pub questioned_at: DateTime<Local>,
}

fn model2dto(question: Question) -> QuestionDTO {
  QuestionDTO {
    question_id: question.id,
    question_body: question.body,
    questioned_at: question.created_at,
  }
}

pub enum PostQuestionError {
  BlankBody
}

trait Usecase {
  fn execute(&self, iport: InputPort<NewQuestionDTO>, oport: OutputPort<Result<QuestionDTO, PostQuestionError>>);
}

mod implement {
  pub struct Usecase {
    question_repository: Box<QuestionRepository>
  }

  impl super::Usecase for Usecase {
    fn execute(&self, iport: InputPort<NewQuestionDTO>, oport: OutputPort<Result<QuestionDTO, PostQuestionError>>) {
      let new_question_dto = iport.input();
      let question = Question {
        id: Uuid::new_v4(),
        body: new_question_dto.body,
        ip_address: new_question_dto.ip_address,
        hidden: false,
      };
      let result = match question.validate() {
        entity::Validation::Valid -> {
          self.question_repository.store(question);
          Ok(model2dto(question))
        },
        entity::Validation::Invalid(entity::QuestionInvalidReason::BlankBody) -> {
          Err(PostQuestionError::BlankBody)
        }
      };
      oport.output(result);
    }
  }
}