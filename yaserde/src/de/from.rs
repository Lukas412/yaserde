use std::fs::File;
use std::io::Read;
use std::path::Path;

use error_stack::{IntoReport, ResultExt};

use crate::de::Deserializer;
use crate::errors::de::DeserializeError;
use crate::errors::de::file::FileDeserializeError;
use crate::errors::de::string::StringDeserializeError;
use crate::errors::string::StringError;
use crate::YaDeserialize;

pub fn from_str<T: YaDeserialize>(s: &str) -> error_stack::Result<T, StringDeserializeError> {
  from_reader(s.as_bytes()).change_context(StringDeserializeError::new(s.to_owned()))
}

pub fn from_file<P: AsRef<Path>, T: YaDeserialize>(path: &P) -> error_stack::Result<T, FileDeserializeError> {
  let file = File::open(path)
    .report().change_context(FileDeserializeError::new(path))?;

  from_reader(file)
    .change_context(FileDeserializeError::new(path))
}

pub fn from_reader<R: Read, T: YaDeserialize>(reader: R) -> error_stack::Result<T, DeserializeError> {
  YaDeserialize::deserialize(&mut Deserializer::new_from_reader(reader))
    .map_err(StringError::new_report)
    .change_context(DeserializeError::default())
}
