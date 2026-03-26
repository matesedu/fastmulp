use fastmulp_core::{Boundary, Error, boundary_from_content_type};

#[test]
fn extracts_boundary_case_insensitively_and_skips_other_params() {
  let content_type = "Multipart/Form-Data; charset=UTF-8; boundary=abc123; foo=bar";
  assert_eq!(boundary_from_content_type(content_type), Some("abc123"));
}

#[test]
fn extracts_quoted_boundary_with_optional_spacing() {
  let content_type = "multipart/form-data; foo=bar; boundary = \"quoted-boundary\"";
  assert_eq!(boundary_from_content_type(content_type), Some("quoted-boundary"));
}

#[test]
fn rejects_invalid_quoted_boundary_values() {
  let content_type = "multipart/form-data; boundary=\"unterminated";
  assert_eq!(boundary_from_content_type(content_type), None);
}

#[test]
fn rejects_missing_boundary_parameter() {
  let content_type = "multipart/form-data; charset=UTF-8";
  assert_eq!(boundary_from_content_type(content_type), None);
}

#[test]
fn validates_boundary_length_and_bytes() {
  assert!(Boundary::new(b"abc123").is_ok());
  assert!(Boundary::new(b"abc def").is_ok());
  assert!(matches!(Boundary::new(b"abc "), Err(Error::InvalidBoundaryByte { .. })));
  assert!(matches!(Boundary::new(b"abc\"123"), Err(Error::InvalidBoundaryByte { .. })));
  assert!(matches!(Boundary::new(&[b'a'; 71]), Err(Error::BoundaryTooLong { .. })));
}

