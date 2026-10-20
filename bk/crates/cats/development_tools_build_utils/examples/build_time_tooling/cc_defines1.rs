#![allow(dead_code)]
// ANCHOR: example
// Declare an external C function that a build script could link in.
unsafe extern "C" {
    fn print_app_info();
}

fn main() {
    let _ffi_symbol: unsafe extern "C" fn() = print_app_info;
    println!(
        "Declared the `print_app_info` symbol. A cc::Build build script can provide the matching C implementation at link time."
    );
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore = "Needs review"]
    #[test]
    fn test_main() {
        main();
    }
}
// [finish; deal with extern](https://github.com/john-cd/rust_howto/issues/901)
