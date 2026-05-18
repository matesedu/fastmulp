#![deny(clippy::unwrap_used, clippy::expect_used)]
#![deny(clippy::undocumented_unsafe_blocks, unsafe_op_in_unsafe_fn)]

use std::{hint::black_box, time::Instant};

use fastmulp_core::parse;

fn main() -> fastmulp_core::Result<()> {
    let boundary = b"----fastmulp-bench";
    let body = make_body();
    let iterations = 20_000;

    let start = Instant::now();
    let mut part_count = 0usize;

    for _ in 0..iterations {
        let multipart = parse(black_box(body.as_slice()), black_box(boundary))?;
        part_count += black_box(multipart.parts().len());
    }

    let elapsed = start.elapsed();
    let throughput = (body.len() * iterations) as f64 / elapsed.as_secs_f64() / (1024.0 * 1024.0);

    println!(
        "iterations={iterations} bytes_per_iteration={} parts={} throughput_mib_per_sec={throughput:.2}",
        body.len(),
        part_count / iterations,
    );

    Ok(())
}

fn make_body() -> Vec<u8> {
    let mut body = Vec::with_capacity(16 * 1024);
    for index in 0..12 {
        body.extend_from_slice(b"------fastmulp-bench\r\n");
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"field");
        body.extend_from_slice(index.to_string().as_bytes());
        body.extend_from_slice(b"\"\r\n\r\n");
        body.extend_from_slice(b"abcdefghijklmnopqrstuvwxyz0123456789");
        body.extend_from_slice(b"\r\n");
    }
    body.extend_from_slice(b"------fastmulp-bench--\r\n");
    body
}
