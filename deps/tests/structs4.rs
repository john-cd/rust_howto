struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    // implementation block (multiple allowed for a given struct)
    // Method
    fn area(&self) -> u32 {
        // short for self: &Self, an alias for the type that the impl block is
        // for
        self.width * self.height
    }

    // Associated Functions - NO self, &self, or &mut self
    // often use for constructors: SomeType::new(...)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let sq = Rectangle::square(5);
    println!("area: {}", sq.area());
}

#[test]
fn test() {
    main();
}
