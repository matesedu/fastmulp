use smallvec::SmallVec;

#[derive(Debug, PartialEq, Eq)]
pub enum TextValue<'a> {
  Borrowed(&'a [u8]),
  Owned(SmallVec<[u8; 32]>),
}

impl<'a> TextValue<'a> {
  pub fn as_bytes(&self) -> &[u8] {
    match self {
      Self::Borrowed(bytes) => bytes,
      Self::Owned(bytes) => bytes.as_slice(),
    }
  }

  pub fn as_str(&self) -> core::result::Result<&str, core::str::Utf8Error> {
    core::str::from_utf8(self.as_bytes())
  }
}
