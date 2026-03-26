use memchr::memchr;
use smallvec::SmallVec;

use crate::{Error, Result, TextValue, util::eq_ignore_ascii_case};

pub(super) fn decode_extended_value<'a>(
    value: TextValue<'a>,
    offset: usize,
) -> Result<TextValue<'a>> {
    match value {
        TextValue::Borrowed(bytes) => decode_extended_borrowed(bytes, offset),
        TextValue::Owned(bytes) => decode_extended_owned(bytes, offset),
    }
}

pub(super) fn decode_html_name_escapes<'a>(value: TextValue<'a>) -> TextValue<'a> {
    match value {
        TextValue::Borrowed(bytes) => match decode_html_name_escape_bytes(bytes) {
            Some(decoded) => TextValue::Owned(decoded),
            None => TextValue::Borrowed(bytes),
        },
        TextValue::Owned(bytes) => match decode_html_name_escape_bytes(bytes.as_slice()) {
            Some(decoded) => TextValue::Owned(decoded),
            None => TextValue::Owned(bytes),
        },
    }
}

fn decode_extended_borrowed<'a>(bytes: &'a [u8], offset: usize) -> Result<TextValue<'a>> {
    let (charset, encoded) = split_extended_value(bytes, offset)?;
    validate_charset(charset, offset)?;

    if memchr(b'%', encoded).is_none() {
        return Ok(TextValue::Borrowed(encoded));
    }

    Ok(TextValue::Owned(percent_decode(encoded, offset)?))
}

fn decode_extended_owned<'a>(bytes: SmallVec<[u8; 32]>, offset: usize) -> Result<TextValue<'a>> {
    let (charset, encoded) = split_extended_value(bytes.as_slice(), offset)?;
    validate_charset(charset, offset)?;

    if memchr(b'%', encoded).is_none() {
        let mut owned = SmallVec::<[u8; 32]>::with_capacity(encoded.len());
        owned.extend_from_slice(encoded);
        return Ok(TextValue::Owned(owned));
    }

    Ok(TextValue::Owned(percent_decode(encoded, offset)?))
}

fn split_extended_value(bytes: &[u8], offset: usize) -> Result<(&[u8], &[u8])> {
    let Some(charset_end) = memchr(b'\'', bytes) else {
        return Err(Error::InvalidContentDisposition { offset });
    };

    let Some(language_len) = memchr(b'\'', &bytes[charset_end + 1..]) else {
        return Err(Error::InvalidContentDisposition { offset });
    };

    let value_start = charset_end + language_len + 2;
    Ok((&bytes[..charset_end], &bytes[value_start..]))
}

fn validate_charset(charset: &[u8], offset: usize) -> Result<()> {
    if eq_ignore_ascii_case(charset, b"utf-8") || eq_ignore_ascii_case(charset, b"us-ascii") {
        return Ok(());
    }

    Err(Error::InvalidContentDisposition { offset })
}

fn percent_decode(bytes: &[u8], offset: usize) -> Result<SmallVec<[u8; 32]>> {
    let mut decoded = SmallVec::<[u8; 32]>::with_capacity(bytes.len());
    let mut cursor = 0;

    while cursor < bytes.len() {
        if bytes[cursor] == b'%' {
            if cursor + 2 >= bytes.len() {
                return Err(Error::InvalidContentDisposition {
                    offset: offset + cursor,
                });
            }

            let hi = decode_hex(bytes[cursor + 1]).ok_or(Error::InvalidContentDisposition {
                offset: offset + cursor + 1,
            })?;
            let lo = decode_hex(bytes[cursor + 2]).ok_or(Error::InvalidContentDisposition {
                offset: offset + cursor + 2,
            })?;
            decoded.push((hi << 4) | lo);
            cursor += 3;
            continue;
        }

        decoded.push(bytes[cursor]);
        cursor += 1;
    }

    Ok(decoded)
}

fn decode_html_name_escape_bytes(bytes: &[u8]) -> Option<SmallVec<[u8; 32]>> {
    let mut cursor = memchr(b'%', bytes)?;
    let mut copied_from = 0;
    let mut decoded = None;

    while cursor + 2 < bytes.len() {
        if bytes[cursor] != b'%' {
            cursor += 1;
            continue;
        }

        let replacement = match &bytes[cursor + 1..cursor + 3] {
            [b'0', b'A' | b'a'] => Some(b'\n'),
            [b'0', b'D' | b'd'] => Some(b'\r'),
            [b'2', b'2'] => Some(b'"'),
            _ => None,
        };

        if let Some(replacement) = replacement {
            let decoded =
                decoded.get_or_insert_with(|| SmallVec::<[u8; 32]>::with_capacity(bytes.len()));
            decoded.extend_from_slice(&bytes[copied_from..cursor]);
            decoded.push(replacement);
            cursor += 3;
            copied_from = cursor;
            continue;
        }

        cursor += 1;
    }

    let mut decoded = decoded?;
    decoded.extend_from_slice(&bytes[copied_from..]);
    Some(decoded)
}

fn decode_hex(byte: u8) -> Option<u8> {
    match byte {
        b'0'..=b'9' => Some(byte - b'0'),
        b'a'..=b'f' => Some(byte - b'a' + 10),
        b'A'..=b'F' => Some(byte - b'A' + 10),
        _ => None,
    }
}
