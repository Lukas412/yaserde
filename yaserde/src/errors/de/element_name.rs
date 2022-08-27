use std::fmt::{Display, Formatter};
use error_stack::Context;

#[derive(Debug)]
pub enum ElementNameError {
  WrongLocalName {
    expected: String,
    actual: String
  },
  WrongNamespace {
    expected: String,
    actual: String
  },
  WrongPrefix {
    expected: String,
    actual: String
  }
}

impl ElementNameError {
  pub fn new_wrong_local_name(expected: String, actual: String) -> Self {
    Self::WrongLocalName { expected, actual }
  }

  pub fn new_wrong_namespace(expected: String, actual: String) -> Self {
    Self::WrongNamespace { expected, actual }
  }

  pub fn new_wrong_prefix(expected: String, actual: String) -> Self {
    Self::WrongPrefix { expected, actual }
  }
}

impl Display for ElementNameError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    match self {
      Self::WrongLocalName { expected, actual} => {
        write!(f, "Element name is wrong. Expected {}, but actually was {}", expected, actual)
      }
      Self::WrongNamespace { expected, actual } => {
        write!(f, "Element namespace is wrong. Expected {}, but actually was {}", expected, actual)
      }
      Self::WrongPrefix { expected, actual } => {
        write!(f, "Element prefix is wrong. Expected {}, but actually was {}", expected, actual)
      }
    }
  }
}

impl Context for ElementNameError {}