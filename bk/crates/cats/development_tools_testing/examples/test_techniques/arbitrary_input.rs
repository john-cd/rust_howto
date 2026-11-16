// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Derive `Arbitrary` to turn raw fuzzer bytes into a structured test case.
//!
//! `Unstructured` hands those bytes to the derive implementation, which reads
//! fields in declaration order. The input borrows its path from the byte slice,
//! so decoding does not need to allocate a `String`. Decoding can fail when the
//! bytes do not contain a valid value; fuzz targets should treat that as an
//! ordinary rejected input. Reusing the same bytes reproduces the same case.
//! The assertion below stands in for an invariant checked by the code under
//! test; a real fuzz target would call `check_input` with each supplied input.

use arbitrary::Arbitrary;
use arbitrary::Unstructured;

#[derive(Arbitrary, Debug)]
struct Request<'a> {
    method: u8,
    path: &'a str,
}

fn check_input(bytes: &[u8]) -> arbitrary::Result<()> {
    let request = Request::arbitrary(&mut Unstructured::new(bytes))?;
    if request.method == 0 && request.path.starts_with('/') {
        assert!(request.path.len() <= bytes.len());
    }
    Ok(())
}

fn main() {
    let bytes = [0, 3, b'/', b'a', b'b', b'c'];
    check_input(&bytes).expect("sample input should decode");
    println!(
        "Structured input: {:?}",
        Request::arbitrary(&mut Unstructured::new(&bytes))
    );
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    #[test]
    fn sample_input_is_reproducible() {
        super::main();
    }
}
