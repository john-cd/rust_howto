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
// ANCHOR_END: example

// TODO
fn main() {}

#[test]
fn test() {
    main();
}
