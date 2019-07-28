use std::sync::{Mutex, Arc};

pub mod repository;
pub mod viewer;
pub mod questioner;
pub mod answerer;

// input port
pub trait InputPort<T> {
  fn input(&self) -> T;
}

pub struct MockInputPort<T: Clone> {
  pub value: T
}

impl<T: Clone> InputPort<T> for MockInputPort<T> {
  fn input(&self) -> T {
    return self.value.clone()
  }
}

// output port
pub trait OutputPort<T> {
  fn output(&self, t: T);
}

#[derive(Clone)]
pub struct MockOutputPort<T> {
  pub value: Arc<Mutex<Option<T>>>
}

impl<T> OutputPort<T> for MockOutputPort<T> {
  fn output(&self, output: T) {
    let mut value = self.value.lock().unwrap();
    *value = Some(output);
  }
}

impl<T> MockOutputPort<T> {
  pub fn new() -> Self {
    Self { value: Arc::new(Mutex::new(None)) }
  }
}
