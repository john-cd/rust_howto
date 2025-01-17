// ANCHOR: example
// ANCHOR_END: example
#![cfg(target_os = "linux")]

fn main() {}

#[test]
#[ignore = "later"]
fn test() {
    main();
}
// [P2](https://github.com/john-cd/rust_howto/issues/818)
