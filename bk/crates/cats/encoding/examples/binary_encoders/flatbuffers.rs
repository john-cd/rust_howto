#![allow(dead_code)]

// ANCHOR: example
//! Build a minimal FlatBuffer payload without a generated schema.
//!
//! TODO Reviewed ??-??-2026

use flatbuffers::FlatBufferBuilder;

fn main() {
    let mut builder = FlatBufferBuilder::new();
    let greeting = builder.create_string("Hello, FlatBuffers!");

    builder.finish_minimal(greeting);

    let finished_bytes = builder.finished_data();
    println!("FlatBuffer size: {}", finished_bytes.len());
    println!("FlatBuffer bytes: {finished_bytes:?}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        main();
    }
    // [finish; https://flatbuffers.dev/languages/rust/](https://github.com/john-cd/rust_howto/issues/1043)
}
