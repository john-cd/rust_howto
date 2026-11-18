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

/// Structured fuzz input whose path borrows directly from the input bytes.
#[derive(Arbitrary, Debug)]
struct Request<'a> {
    /// Arbitrary method discriminator; zero enables the sample invariant check.
    method: u8,
    /// UTF-8 path borrowed from the decoder's input buffer.
    path: &'a str,
}

/// Decodes fuzz bytes and checks a sample invariant for method-zero absolute paths.
///
/// Returns a decoding error when the bytes cannot form a valid Request.
fn check_input(bytes: &[u8]) -> arbitrary::Result<()> {
    let request = Request::arbitrary(&mut Unstructured::new(bytes))?;
    if request.method == 0 && request.path.starts_with('/') {
        assert!(request.path.len() <= bytes.len());
    }
    Ok(())
}

/// Runs the example demonstrated by this module.
fn main() {
    let bytes = [0, 3, b'/', b'a', b'b', b'c'];
    check_input(&bytes).expect("sample input should decode");
    println!(
        "Structured input: {:?}",
        Request::arbitrary(&mut Unstructured::new(&bytes))
    );
}
// ANCHOR_END: example

/// Exercises the fixed fuzz input through the example's decoding path.
#[cfg(test)]
mod tests {
    /// Checks that sample input is reproducible.
    #[test]
    fn sample_input_is_reproducible() {
        super::main();
    }
}
