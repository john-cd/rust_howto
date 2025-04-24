// ANCHOR: example
use std::fmt;

// A supertrait is a trait that another trait depends on.
// In this case, `OutlinePrint` depends on `Display`.
// It means that any type that implements `OutlinePrint` must also implement
// `Display`.
//
// BEWARE: supertraits are NOT inheritance!
// They are only a constraint mechanism. When you define a trait with a
// supertrait, you're essentially saying, "Any type that implements this trait
// must also implement these other traits."  In inheritance, a subclass can
// inherit data (fields) and default implementations from its superclass. This
// is not true in Rust.

trait OutlinePrint: fmt::Display {
    fn outline_print(&self) {
        println!("* {} *", self);
        // We can use `println!` here,
        // because `self` is guaranteed to implement `Display`.
    }
}

/// Implement the `OutlinePrint` trait for `String`.
/// `String` already implements the `Display` supertrait. That would not work
/// otherwise.
impl OutlinePrint for String {}

fn main() {
    String::from("test").outline_print();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
