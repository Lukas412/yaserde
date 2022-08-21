use std::fmt::{Display, Formatter};
use error_stack::{Context, Report, report};
use xml::reader::XmlEvent;

#[derive(Debug)]
pub struct UnexpectedEventError {
  event: XmlEvent
}

impl UnexpectedEventError {
  pub fn new_report(event: XmlEvent) -> Report<Self> {
    report!(Self::new(event))
  }

  pub fn new(event: XmlEvent) -> Self {
    Self { event }
  }
}

impl Display for UnexpectedEventError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Unexpected Event: {:?}", self.event)
  }
}

impl Context for UnexpectedEventError {}