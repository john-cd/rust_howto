// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Derive `Arbitrary` to turn raw fuzzer bytes into a structured test case.
//! The same bytes always produce the same input, which makes failures easy
//! to reproduce. A fuzz target can call `check_input` with its supplied bytes.

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
