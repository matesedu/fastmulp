pub(crate) fn eq_ignore_ascii_case(left: &[u8], right: &[u8]) -> bool {
  left.len() == right.len()
    && left
      .iter()
      .zip(right.iter())
      .all(|(left, right)| left.eq_ignore_ascii_case(right))
}

pub(crate) fn trim_ascii(bytes: &[u8]) -> &[u8] {
  trim_ascii_end(trim_ascii_start(bytes))
}

pub(crate) fn skip_ascii_whitespace(bytes: &[u8], mut cursor: usize) -> usize {
  while cursor < bytes.len() && matches!(bytes[cursor], b' ' | b'\t') {
    cursor += 1;
  }

  cursor
}

fn trim_ascii_start(mut bytes: &[u8]) -> &[u8] {
  while let Some(first) = bytes.first() {
    if !matches!(first, b' ' | b'\t') {
      break;
    }

    bytes = &bytes[1..];
  }

  bytes
}

fn trim_ascii_end(mut bytes: &[u8]) -> &[u8] {
  while let Some(last) = bytes.last() {
    if !matches!(last, b' ' | b'\t') {
      break;
    }

    bytes = &bytes[..bytes.len() - 1];
  }

  bytes
}
