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

#[ignore]

fn main() {}

#[test]
fn test() {
    main();
}
