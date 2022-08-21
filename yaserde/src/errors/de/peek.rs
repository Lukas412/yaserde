use std::fmt::{Display, Formatter};
use error_stack::{Context, report};

#[derive(Debug, Default)]
pub struct CouldNotPeekError {}

impl CouldNotPeekError {
  pub fn new_report() -> error_stack::Report<Self> {
    report!(Self::default())
  }
}

impl Display for CouldNotPeekError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "could not peek next event")
  }
}

impl Context for CouldNotPeekError {}
