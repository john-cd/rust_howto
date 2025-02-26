// ANCHOR: example
use ryu::Buffer;

// Pure Rust implementation of Ryū, an algorithm to quickly convert floating
// point numbers to decimal strings.

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
