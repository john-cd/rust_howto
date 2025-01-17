// ANCHOR: example
#![cfg(target_os = "freebsd")]

fn main() {}
// ANCHOR_END: example

#[test]
#[ignore = "later"]
fn test() {
    main();
}
// [P2](https://github.com/john-cd/rust_howto/issues/817)
