use std::io::Read;

use error_stack::{IntoReport, ResultExt};
use xml::{EventReader, ParserConfig};
use xml::name::OwnedName;
use xml::reader::XmlEvent;

use crate::errors::de::DeserializeError;
use crate::errors::de::element_tags::ElementTagsError;
use crate::errors::de::peek::CouldNotPeekError;
use crate::errors::de::unexpected_event::UnexpectedEventError;

pub struct Deserializer<R: Read> {
  depth: usize,
  reader: EventReader<R>,
  peeked: Option<XmlEvent>,
}

impl<R: Read> Deserializer<R> {
  pub fn new(reader: EventReader<R>) -> Self {
    Deserializer {
      depth: 0,
      reader,
      peeked: None,
    }
  }

  pub fn new_from_reader(reader: R) -> Self {
    let config = ParserConfig::new()
      .trim_whitespace(true)
      .whitespace_to_characters(true)
      .cdata_to_characters(true)
      .ignore_comments(true)
      .coalesce_characters(true);

    Self::new(EventReader::new_with_config(reader, config))
  }

  pub fn peek(&mut self) -> error_stack::Result<&XmlEvent, DeserializeError> {
    if self.peeked.is_none() {
      let next = self.inner_next()
        .change_context(DeserializeError::default())?;

      self.peeked = Some(next);
    }

    self.peeked.as_ref()
      .ok_or(CouldNotPeekError::new_report())
      .change_context(DeserializeError::default())
  }

  pub fn inner_next(&mut self) -> error_stack::Result<XmlEvent, xml::reader::Error> {
    loop {
      let next = self.reader.next().report();

      if matches!(next,
        Ok(XmlEvent::StartDocument { .. }
           | XmlEvent::ProcessingInstruction { .. }
           | XmlEvent::Comment(_))) {
        continue;
      }

      return next;
    }
  }

  pub fn next_event(&mut self) -> error_stack::Result<XmlEvent, DeserializeError> {
    let next_event =
      match self.peeked.take() {
        Some(peeked) => peeked,
        None => self.inner_next()
          .change_context(DeserializeError::default())?
      };

    match next_event {
      XmlEvent::StartElement { .. } => {
        self.depth += 1;
      }
      XmlEvent::EndElement { .. } => {
        self.depth -= 1;
      }
      _ => {}
    }

    log::debug!("Fetched {:?}, new depth {}", next_event, self.depth);
    Ok(next_event)
  }

  pub fn skip_element(&mut self, mut cb: impl FnMut(&XmlEvent)) -> error_stack::Result<(), DeserializeError> {
    let depth = self.depth;

    while self.depth >= depth {
      cb(&self.next_event()?);
    }

    Ok(())
  }

  pub fn depth(&self) -> usize {
    self.depth
  }

  pub fn read_inner_value<T, F: FnOnce(&mut Self) -> error_stack::Result<T, DeserializeError>>(
    &mut self,
    f: F,
  ) -> error_stack::Result<T, DeserializeError> {
    match self.next_event()? {
      XmlEvent::StartElement { name, .. } => {
        let result = f(self)?;
        self.expect_end_element(name)?;
        Ok(result)
      }
      event => Err(UnexpectedEventError::new_report(event))
        .change_context(DeserializeError::default())
    }
  }

  pub fn expect_end_element(&mut self, start_name: OwnedName) -> error_stack::Result<(), DeserializeError> {
    let end_name =
      match self.next_event()? {
        XmlEvent::EndElement { name, .. } => name,
        event => {
          return Err(UnexpectedEventError::new_report(event))
            .change_context(DeserializeError::default());
        }
      };

    if end_name != start_name {
      return Err(ElementTagsError::new_report(start_name, end_name))
        .change_context(DeserializeError::default());
    }

    Ok(())
  }
}