use core::ops::Range;

use smallvec::SmallVec;

use crate::{Header, TextValue};

#[derive(Debug)]
pub struct Part<'a> {
  headers: SmallVec<[Header<'a>; 4]>,
  body_start: usize,
  body_end: usize,
  name: Option<TextValue<'a>>,
  file_name: Option<TextValue<'a>>,
  content_type: Option<&'a [u8]>,
}

impl<'a> Part<'a> {
  pub(crate) fn new(
    headers: SmallVec<[Header<'a>; 4]>,
    body_start: usize,
    body_end: usize,
    name: Option<TextValue<'a>>,
    file_name: Option<TextValue<'a>>,
    content_type: Option<&'a [u8]>,
  ) -> Self {
    Self {
      headers,
      body_start,
      body_end,
      name,
      file_name,
      content_type,
    }
  }

  pub fn headers(&self) -> &[Header<'a>] {
    &self.headers
  }

  pub fn body_range(&self) -> Range<usize> {
    self.body_start..self.body_end
  }

  pub fn body<'b>(&self, source: &'b [u8]) -> &'b [u8] {
    &source[self.body_start..self.body_end]
  }

  pub fn name(&self) -> Option<&TextValue<'a>> {
    self.name.as_ref()
  }

  pub fn file_name(&self) -> Option<&TextValue<'a>> {
    self.file_name.as_ref()
  }

  pub fn content_type(&self) -> Option<&'a [u8]> {
    self.content_type
  }
}
