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

// TODO
fn main() {}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
