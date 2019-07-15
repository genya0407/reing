pub mod repository;
pub mod viewer;

trait InputPort<T> {
  fn input(&self) -> T;
}

trait OutputPort<T> {
  fn output(&self, t: T);
}