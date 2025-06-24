#![allow(dead_code)]
// ANCHOR: example
/// A struct representing a rectangle with a width and height.
struct Rectangle {
    width: u32,
    height: u32,
}

/// Implementation block (multiple blocks are allowed for a given struct).
impl Rectangle {
    /// Method: note the paramater that reference `Self`, an alias
    /// for the type that the `impl` block is for, in this case `Rectangle`.
    fn area(&self) -> u32 {
        // `&self` is short for `self: &Self`, that a function paramater that is
        // an immutable reference to `Self`.
        self.width * self.height
    }

    /// Associated Function.
    /// Note that there are NO `self`, `&self`, or `&mut self`.
    /// This is often used to define a "constructor": `SomeType::new(...)`.
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    // Call the associated function.
    // Note the syntax: `<Type>::<function>`.
    let sq: Rectangle = Rectangle::square(5);

    // Call the method. Note the dot operator.
    println!("Area: {}", sq.area());
    // This is equivalent to `Rectangle::area(&sq)`.
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
