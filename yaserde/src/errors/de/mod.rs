use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};

pub mod string;
pub mod file;

#[derive(Debug, Default)]
pub struct DeserializeError;

impl DeserializeError {
  pub fn new_report() -> Report<Self> {
    report!(Self::default())
  }
}

impl Display for DeserializeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error while deserializing xml")
  }
}

impl Context for DeserializeError {}