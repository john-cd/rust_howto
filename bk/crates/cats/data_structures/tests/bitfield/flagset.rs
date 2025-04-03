// ANCHOR: example
//! Example of using the `flagset` crate to define and use bitflags.
//!
//! FlagSet has no dependencies, incl. the stdlib, so it can be used in `no_std`
//! environments, such as embedded systems or kernel development.
use std::os::raw::c_int;

use flagset::FlagSet;
use flagset::flags;

// Flags are defined using the `flags!` macro:
flags! {
    // Flag values can be defined implicitly
    enum FlagsA: u8 { // Note the field-size type
        Foo, // 0b0001
        Bar, // 0b0010
        Baz, // 0b0100
    }

    // Flag values can also be defined explicitly...
    enum FlagsB: c_int {
        Foo = 0x01,
        Bar = 2,
        Baz = 0b0110, // ...and overlap.
        All = (FlagsB::Foo | FlagsB::Bar | FlagsB::Baz).bits(),
    }
}
// Attributes can be used on the enumeration itself or any of the values.

// A collection of flags is a FlagSet<T>.
#[derive(Debug)]
struct Container(FlagSet<FlagsA>);

impl Container {
    fn new(flags: impl Into<FlagSet<FlagsA>>) -> Container {
        Container(flags.into())
    }
}

fn main() {
    let container = Container::new(FlagsA::Foo | FlagsA::Bar);
    assert_eq!(container.0.bits(), 0b011);
    println!("{:?}", container);

    assert_eq!(Container::new(None).0.bits(), 0b000);
}
// Adapted from https://docs.rs/flagset/
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
