// <https://doc.rust-lang.org/cargo/reference/build-scripts.html>
#![allow(dead_code)]

// ANCHOR: C
fn c() {
    // Compile `examples/hello.c` into `libhello.a`
    cc::Build::new().file("examples/hello.c").compile("hello");
}
// ANCHOR_END: C

// ANCHOR: CPP
fn cpp() {
    // Compile `src/foo.cpp` into `libfoo.a`
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=examples/foo.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");
}
// ANCHOR_END: CPP

// ANCHOR: C_DEFINES
fn cc_defines() {
    // Compile `examples/foo.c` into `libfoo.a` with defines
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define(
            "VERSION",
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
        )
        .define("WELCOME", None)
        .file("examples/foo.c")
        .compile("foo");
}
// ANCHOR_END: C_DEFINES

fn main() {
    // c();
    // cc_defines();
    // cpp();
}
// [fix](https://github.com/john-cd/rust_howto/issues/998)
