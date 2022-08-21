//! Generic data structure deserialization framework.
//!

pub use from::{from_file, from_reader, from_str};
pub use deserializer::Deserializer;

mod from;
mod deserializer;
