#![allow(unused)]
// ANCHOR: example
//! `bitflags` generate types for C-style flags with ergonomic APIs.
use std::fmt;

use bitflags::bitflags;

// Define a flag type using the `bitflags` macro.
bitflags! {
    // Attributes can be applied to flags types:
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    // You can derive additional traits on generated flags types if you enable the corresponding features:
    // #[derive(Serialize, Deserialize)]
    pub struct MyFlags: u32 {
        // A flag is a set of bits that may have a unique name.
        const FLAG_A       = 0b0000_0001;
        const FLAG_B       = 0b0000_0010; // Or 1 << 1;
        const FLAG_C       = 0b0000_0100;
        // Flags that set multiple bits are possible:
        const FLAG_ABC     = Self::FLAG_A.bits()
                           | Self::FLAG_B.bits()
                           | Self::FLAG_C.bits();
        // Any bits in a flag you define are called "known bits". Any other bits are "unknown bits".
        // Some operators will unset any unknown bits they encounter
        // If you're generating flags types for an external source, such as a C API,
        // you can define an extra unnamed flag as a mask of all bits the external source may ever set.
        // const _ = !0;
    }
}

// `impl` blocks can be added to flags types to implement additional methods.
impl MyFlags {
    // Convert the bitflags to a u64:
    pub fn as_u64(&self) -> u64 {
        self.bits() as u64
    }
}

// Implement the `Display` trait for `MyFlags` to customize its string
// representation:
impl fmt::Display for MyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits())
    }
}

fn main() {
    // Demonstrate bitwise operations with the defined flags.
    // Union:
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 | e2), MyFlags::FLAG_ABC);
    // Intersection:
    assert_eq!((e1 & e2), MyFlags::FLAG_C);
    // Difference:
    assert_eq!((e1 - e2), MyFlags::FLAG_A);
    // Complement:
    assert_eq!(!e2, MyFlags::FLAG_A);
    // Use the `fmt::Display` implementation above:
    println!("e1: {e1} e2: {e2}");

    // Demonstrate formatting options for the bitflags:
    let flags = MyFlags::FLAG_ABC;
    assert_eq!(format!("{flags}"), "00000000000000000000000000000111");
    assert_eq!(format!("{:?}", MyFlags::FLAG_B), "MyFlags(FLAG_B)");
    assert_eq!(
        format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B),
        "MyFlags(FLAG_A | FLAG_B)"
    );
    println!("{flags:?}");

    // Bitwise pattern matching:
    bitflags::bitflags_match!(flags, {
        MyFlags::FLAG_A | MyFlags::FLAG_B => println!("The value is A and B"),
        _ => println!("The value is not A and B"),
    });
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
