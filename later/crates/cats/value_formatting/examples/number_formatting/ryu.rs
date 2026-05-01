#![allow(dead_code)]
// ANCHOR: example
//! # Ryū Example
//!
//! This example demonstrates the usage of the `ryu` crate, a pure Rust
//! implementation of the Ryū algorithm for fast floating-point to decimal
//! string conversion.

use ryu::Buffer;

fn main() {
    let mut buffer = Buffer::new();
    let printed = buffer.format(1.234);
    assert_eq!(printed, "1.234");

    let mut buffer = Buffer::new();
    let printed = buffer.format(1.234e10);
    assert_eq!(printed, "12340000000.0");

    let mut buffer = Buffer::new();
    let printed = buffer.format(1.234e-10);
    assert_eq!(printed, "1.234e-10");

    let mut buffer = Buffer::new();
    let printed = buffer.format(1.234e-1);
    assert_eq!(printed, "0.1234");

    let mut buffer = Buffer::new();
    let printed = buffer.format(1.234e-2);
    assert_eq!(printed, "0.01234");

    let mut buffer = Buffer::new();
    let printed = buffer.format(1.234e-3);
    assert_eq!(printed, "0.001234");

    let mut buffer = Buffer::new();
    let printed = buffer.format(1.234e-7);
    assert_eq!(printed, "1.234e-7");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
