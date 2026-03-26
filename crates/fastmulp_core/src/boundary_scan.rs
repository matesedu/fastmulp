use memchr::memchr;

use crate::{Error, Result};

pub(crate) enum BoundaryLine {
    Encapsulation { next_cursor: usize },
    Close,
}

pub(crate) fn find_next_boundary(
    body: &[u8],
    boundary: &[u8],
    from: usize,
) -> Result<(usize, usize, bool)> {
    let mut cursor = from;

    while let Some(boundary_start) = find_boundary_start(body, boundary, cursor) {
        let boundary_end = boundary_start + boundary.len() + 4;

        let Some(line) = classify_boundary_line(body, boundary_end) else {
            cursor = boundary_start + 1;
            continue;
        };

        match line {
            BoundaryLine::Encapsulation { next_cursor } => {
                return Ok((boundary_start, next_cursor, false));
            }
            BoundaryLine::Close => {
                return Ok((boundary_start, body.len(), true));
            }
        }
    }

    Err(Error::MissingClosingBoundary { offset: from })
}

pub(crate) fn find_boundary_start(body: &[u8], boundary: &[u8], mut from: usize) -> Option<usize> {
    let marker_len = boundary.len() + 4;

    while from < body.len() {
        let relative = memchr(b'\r', &body[from..])?;
        let candidate = from + relative;
        let remaining = &body[candidate..];
        if remaining.len() < marker_len {
            return None;
        }
        if remaining[1] == b'\n'
            && remaining[2] == b'-'
            && remaining[3] == b'-'
            && remaining[4..].starts_with(boundary)
        {
            return Some(candidate);
        }
        from = candidate + 1;
    }

    None
}

pub(crate) fn classify_boundary_line(body: &[u8], cursor: usize) -> Option<BoundaryLine> {
    if body[cursor..].starts_with(b"--") {
        let padded = skip_transport_padding(body, cursor + 2);
        if padded == body.len() || body[padded..].starts_with(b"\r\n") {
            return Some(BoundaryLine::Close);
        }

        return None;
    }

    let padded = skip_transport_padding(body, cursor);
    if body[padded..].starts_with(b"\r\n") {
        return Some(BoundaryLine::Encapsulation {
            next_cursor: padded + 2,
        });
    }

    None
}

fn skip_transport_padding(body: &[u8], mut cursor: usize) -> usize {
    while cursor < body.len() && matches!(body[cursor], b' ' | b'\t') {
        cursor += 1;
    }

    cursor
}
