use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};

mod string;
mod file;
mod unexpected_event;
mod element_tags;
mod peek;

pub use string::StringDeserializeError;
pub use file::FileDeserializeError;
pub use unexpected_event::UnexpectedEventError;
pub use element_tags::ElementTagsError;
pub use peek::CouldNotPeekError;

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