use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};

#[derive(Debug)]
pub struct StringDeserializeError {
  text: String
}

impl StringDeserializeError {
  pub fn new_report(text: String) -> Report<Self> {
    report!(Self::new(text))
  }

  pub fn new(text: String) -> Self {
    Self { text }
  }
}

impl Display for StringDeserializeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error while deserializing xml from string: {}", self.text)
  }
}

impl Context for StringDeserializeError {}