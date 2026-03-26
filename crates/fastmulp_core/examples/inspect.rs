#![deny(clippy::unwrap_used, clippy::expect_used)]
#![deny(clippy::undocumented_unsafe_blocks, unsafe_op_in_unsafe_fn)]

use fastmulp_core::parse;

fn main() -> Result<(), Box<dyn std::error::Error>> {
  let boundary = "demo-boundary";
  let body = concat!(
    "--demo-boundary\r\n",
    "Content-Disposition: form-data; name=\"field\"\r\n",
    "\r\n",
    "hello world\r\n",
    "--demo-boundary--\r\n",
  );

  let multipart = parse(body.as_bytes(), boundary.as_bytes())?;
  for part in multipart.parts() {
    let name = part.name().and_then(|value| value.as_str().ok()).unwrap_or("-");
    let value = core::str::from_utf8(part.body(multipart.body()))?;
    println!("{name}: {value}");
  }

  Ok(())
}
