# fastmulp

High-accuracy, low-allocation `multipart/form-data` parsing with a zero-copy Rust core and thin targets for Node.js and the browser.

## Workspace

- `crates/fastmulp_core`: zero-copy parser and native API
- `crates/fastmulp_napi`: Node.js bindings built with `napi-rs`
- `crates/fastmulp_wasm`: browser bindings built with `wasm-bindgen`

## Design

- The Rust core parses against a borrowed `&[u8]` and returns body ranges instead of copying payload bytes.
- `Content-Disposition` is parsed eagerly, and `name` is enforced for `form-data` parts.
- Header storage uses `SmallVec` so the common case stays stack-friendly.
- Node.js and browser bindings return metadata plus `body_start` / `body_end`, so callers can slice the original buffer themselves.
- Boundary lines accept RFC 2046 transport padding, plus MIME-style preamble and epilogue.

## Spec Notes

- [RFC 7578 Section 4.1](https://www.rfc-editor.org/rfc/rfc7578#section-4.1): boundary handling
- [RFC 7578 Section 4.2](https://www.rfc-editor.org/rfc/rfc7578#section-4.2): `Content-Disposition` requirements for each part
- [RFC 7578 Section 4.3](https://www.rfc-editor.org/rfc/rfc7578#section-4.3): multiple files and older nested `multipart/mixed`
- [RFC 2046 Section 5.1.1](https://www.rfc-editor.org/rfc/rfc2046#section-5.1.1): multipart syntax, transport padding, preamble, epilogue
- [HTML multipart/form-data algorithm](https://html.spec.whatwg.org/multipage/form-control-infrastructure.html#multipart-form-data): browser-side escaping rules for names and filenames

For deployed compatibility, `fastmulp` also accepts `filename*=` extended parameters even though [RFC 7578 Section 4.2](https://www.rfc-editor.org/rfc/rfc7578#section-4.2) says senders must not generate them.

`filename` values are untrusted input. Follow the security guidance in [RFC 7578 Section 4.2](https://www.rfc-editor.org/rfc/rfc7578#section-4.2) and avoid using path components blindly.

## Rust Example

```rust
use fastmulp_core::parse;

let boundary = "demo-boundary";
let body = b"--demo-boundary\r\nContent-Disposition: form-data; name=\"field\"\r\n\r\nhello\r\n--demo-boundary--\r\n";
let multipart = parse(body, boundary.as_bytes())?;

let part = &multipart.parts()[0];
assert_eq!(part.name().and_then(|value| value.as_str().ok()), Some("field"));
assert_eq!(part.body(multipart.body()), b"hello");
# Ok::<(), fastmulp_core::Error>(())
```

## JS Targets

Node.js:

```ts
import { parse } from "./fastmulp.node";

const parts = parse(bodyBuffer, boundary);
const fileBytes = bodyBuffer.subarray(parts[0].body_start, parts[0].body_end);
```

Browser:

```ts
import init, { parse } from "./fastmulp_wasm.js";

await init();
const parts = parse(formBytes, boundary);
const fieldBytes = formBytes.subarray(parts[0].body_start, parts[0].body_end);
```

The wasm target still needs one JS-to-wasm copy at the ABI boundary, but it avoids extra copies after parsing by returning ranges instead of materialized part bodies.

Older nested `multipart/mixed` payloads can be handled by recursively calling `parse` on a part body after extracting the nested boundary from that part's `Content-Type`.

## Release

- `vp run release:patch`
- `vp run release:minor`
- `vp run release:alpha`
- `vp run release:beta`

Each release command updates the workspace version in `Cargo.toml`, creates a release commit, and creates the matching `v...` git tag. Tag pushes publish `fastmulp-core` through GitHub Actions trusted publishing.

## Shared Tasks

- `vp run fmt`
- `vp run fmt:check`
- `vp run ci:local`
- `vp run lint`
- `vp run check`
- `vp run test`
- `vp run bench`

## License

`fastmulp` is licensed under `GPL-3.0-or-later`. See `LICENSE` and the [GNU GPL v3 text](https://www.gnu.org/licenses/gpl-3.0.txt).
