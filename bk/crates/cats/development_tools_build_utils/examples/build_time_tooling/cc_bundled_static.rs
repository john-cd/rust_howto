#![allow(dead_code)]
// ANCHOR: example
/// This example demonstrates how to use the `cc` crate to compile a C file into
/// a static library.
///
/// The `cc` crate is a build dependency that allows compiling C/C++ code
/// as part of your Rust build process.
///
/// In this example, we're compiling `src/hello.c` into a static library named
/// `libhello.a`.
///
/// Note that this example is commented out because it requires a C file to be
/// present in the `src` directory.
fn main() {
    // TODO
    // cc::Build::new().file("src/hello.c").compile("hello");
    // // outputs `libhello.a`
    println!(
        "cc_bundled_static example: compiles a C file into a static library (requires a C compiler)"
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    #[ignore = "Needs review"]
    fn test() {
        main();
    }
}
// [finish; deal with cc](https://github.com/john-cd/rust_howto/issues/899)
