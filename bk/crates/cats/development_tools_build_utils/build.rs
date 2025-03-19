// https://doc.rust-lang.org/cargo/reference/build-scripts.html
#![allow(dead_code)]

// ANCHOR: C
fn c() {
    cc::Build::new().file("tests/hello.c").compile("hello");
    // outputs `libhello.a`
}
// ANCHOR_END: C

// ANCHOR: CPP
fn cpp() {
    // Tell Cargo that if the given file changes, to rerun this build script.
    println!("cargo::rerun-if-changed=tests/foo.cpp");
    cc::Build::new()
        .cpp(true)
        .file("src/foo.cpp")
        .compile("foo");
}
// ANCHOR_END: CPP

// ANCHOR: C_DEFINES
fn cc_defines() {
    cc::Build::new()
        .define("APP_NAME", "\"foo\"")
        .define(
            "VERSION",
            format!("\"{}\"", env!("CARGO_PKG_VERSION")).as_str(),
        )
        .define("WELCOME", None)
        .file("tests/foo.c")
        .compile("foo");
}
// ANCHOR_END: C_DEFINES

fn main() {
    // c();
    // cc_defines();
    // cpp();
}
// [WIP fix SOON](https://github.com/john-cd/rust_howto/issues/998)
