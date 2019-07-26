pub mod repository;
pub mod viewer;
pub mod questioner;
pub mod answerer;

pub trait InputPort<T> {
  fn input(&self) -> T;
}

pub trait OutputPort<T> {
  fn output(&self, t: T);
}