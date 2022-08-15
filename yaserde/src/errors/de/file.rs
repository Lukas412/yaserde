use std::fmt::{Debug, Display, Formatter};
use std::path::{Path, PathBuf};
use error_stack::{Context, Report, report};

#[derive(Debug)]
pub struct FileDeserializeError {
  filename: PathBuf
}

impl FileDeserializeError {
  pub fn new_report<P: AsRef<Path>>(filename: P) -> Report<Self> {
    report!(Self::new(filename))
  }

  pub fn new<P: AsRef<Path>>(filename: P) -> Self {
    let filename = filename.as_ref().into();
    Self { filename }
  }
}

impl Display for FileDeserializeError {
  fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
    write!(f, "Error while deserializing xml from file: {:?}", self.filename)
  }
}

impl Context for FileDeserializeError {}