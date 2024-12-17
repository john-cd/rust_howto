// ANCHOR: example
use std::os::raw::c_int;

use flagset::flags;

flags! {
    enum FlagsA: u8 {
        Foo,
        Bar,
        Baz,
    }

    enum FlagsB: c_int {
        Foo,
        Bar,
        Baz,
    }
}

fn main() {}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}

// [flagset: main (P1)](https://github.com/john-cd/rust_howto/issues/153)
