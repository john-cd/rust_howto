#![allow(unused)]
// ANCHOR: example
use std::fmt;

use bitflags::bitflags;

bitflags! {
    // Attributes can be applied to flags types
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    struct MyFlags: u32 {
        const FLAG_A       = 0b0000_0001;
        const FLAG_B       = 0b0000_0010;
        const FLAG_C       = 0b0000_0100;
        const FLAG_ABC     = Self::FLAG_A.bits()
                           | Self::FLAG_B.bits()
                           | Self::FLAG_C.bits();
    }
}

impl MyFlags {
    pub fn as_u64(&self) -> u64 {
        self.bits() as u64
    }
}

impl fmt::Display for MyFlags {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:032b}", self.bits())
    }
}

fn main() {
    let e1 = MyFlags::FLAG_A | MyFlags::FLAG_C;
    let e2 = MyFlags::FLAG_B | MyFlags::FLAG_C;
    assert_eq!((e1 | e2), MyFlags::FLAG_ABC);
    assert_eq!((e1 & e2), MyFlags::FLAG_C);
    assert_eq!((e1 - e2), MyFlags::FLAG_A);
    assert_eq!(!e2, MyFlags::FLAG_A);

    let flags = MyFlags::FLAG_ABC;
    assert_eq!(format!("{}", flags), "00000000000000000000000000000111");
    assert_eq!(format!("{:?}", MyFlags::FLAG_B), "MyFlags(FLAG_B)");
    assert_eq!(
        format!("{:?}", MyFlags::FLAG_A | MyFlags::FLAG_B),
        "MyFlags(FLAG_A | FLAG_B)"
    );
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// TODO P1
