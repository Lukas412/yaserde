use std::fmt::{Display, Formatter};
use error_stack::{Context, report};
use xml::name::OwnedName;

#[derive(Debug)]
pub struct ElementTagsError {
  start_name: OwnedName,
  end_name: OwnedName
}

impl ElementTagsError {
  pub fn new_report(start_name: OwnedName, end_name: OwnedName) -> error_stack::Report<Self> {
    report!(Self::new(start_name, end_name))
  }

  pub fn new(start_name: OwnedName, end_name: OwnedName) -> Self {
    Self { start_name, end_name }
  }
}

impl Display for ElementTagsError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Wrong element tags: <{}> ... </{}>", self.start_name.local_name, self.end_name.local_name)
  }
}

impl Context for ElementTagsError {}
