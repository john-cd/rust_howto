// ANCHOR: example
#[derive(Debug, Clone, Copy)] // All fields (i32) are Copy, so Point can be Copy
struct Point2D {
    x: i32,
    y: i32,
}

// Or, a manual implementation:
// use std::fmt;
// impl fmt::Debug for Point {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("Point")
//          .field("x", &self.x)
//          .field("y", &self.y)
//          .finish()
//     }
// }

// Display Trait
// Allows your struct to be printed using the {} format specifier, for
// user-facing output.
use std::fmt;

impl fmt::Display for Point2D {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point2D: {}, {}", self.x, self.y)
    }
}

fn main() {
    let p = Point2D { x: 1, y: 1 };
    println!("{:?}", p);
    println!("{}", p);

    let p2 = p.clone();
    println!("{:?}", p2);

    let p3 = p2;
    println!("{:?}", p3);
}

// ANCHOR_END: example

#[test]
fn test() {
    main();
}
