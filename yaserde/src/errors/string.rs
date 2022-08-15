use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};

#[derive(Debug)]
pub(crate) struct StringError(String);

impl StringError {
  pub fn new_report(text: String) -> Report<Self> {
    report!(Self::new(text))
  }

  pub fn new(text: String) -> Self {
    Self(text)
  }
}

impl Display for StringError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error: '{}'", self.0)
  }
}

impl Context for StringError {}
