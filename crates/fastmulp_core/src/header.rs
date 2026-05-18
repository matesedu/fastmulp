use memchr::memchr;

use crate::{
    Error, Result,
    util::{eq_ignore_ascii_case, trim_ascii},
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Header<'a> {
    name: &'a [u8],
    value: &'a [u8],
}

impl<'a> Header<'a> {
    pub fn new(name: &'a [u8], value: &'a [u8]) -> Self {
        Self { name, value }
    }

    pub fn name(&self) -> &'a [u8] {
        self.name
    }

    pub fn value(&self) -> &'a [u8] {
        self.value
    }

    pub fn name_eq_ignore_ascii_case(&self, other: &[u8]) -> bool {
        eq_ignore_ascii_case(self.name, other)
    }

    pub(crate) fn parse(line: &'a [u8], offset: usize) -> Result<Self> {
        let Some(separator) = memchr(b':', line) else {
            return Err(Error::MissingHeaderSeparator { offset });
        };

        let name = &line[..separator];
        if name.is_empty() || !name.iter().copied().all(is_tchar) {
            return Err(Error::InvalidHeaderName { offset });
        }

        let value = trim_ascii(&line[separator + 1..]);
        Ok(Self { name, value })
    }
}

fn is_tchar(byte: u8) -> bool {
    matches!(
      byte,
      b'!' | b'#'
        | b'$'
        | b'%'
        | b'&'
        | b'\''
        | b'*'
        | b'+'
        | b'-'
        | b'.'
        | b'^'
        | b'_'
        | b'`'
        | b'|'
        | b'~'
        | b'0'..=b'9'
        | b'a'..=b'z'
        | b'A'..=b'Z'
    )
}
