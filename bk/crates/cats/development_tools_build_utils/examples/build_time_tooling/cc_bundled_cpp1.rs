// TODO Reviewed ??-??-2026
#![allow(dead_code)]
// ANCHOR: example
// Declare a C-compatible symbol that can be implemented in bundled C++ code.
unsafe extern "C" {
    fn multiply(x: i32, y: i32) -> i32;
}

fn main() {
    let _ffi_symbol: unsafe extern "C" fn(i32, i32) -> i32 = multiply;
    println!(
        "Declared the `multiply` symbol. A build script can compile and link the corresponding C++ implementation."
    );
}
// ANCHOR_END: example

/// Runs the example.
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
    // [finish](https://github.com/john-cd/rust_howto/issues/898)
}
