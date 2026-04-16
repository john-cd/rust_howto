#![allow(dead_code)]
// ANCHOR: example

// `#[derive(PartialEq, Eq)]` automatically implements equality traits.
// When derived on structs, two instances are equal if all fields are equal,
// and not equal if any fields are not equal.
// When derived on enums, two instances are equal if they are the same variant
// and all fields are equal.
#[derive(Debug, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

fn points() {
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 1, y: 2 };
    let p3 = Point { x: 3, y: 4 };

    println!("p1 == p2: {}", p1 == p2); // true
    println!("p1 == p3: {}", p1 == p3); // false
}

#[derive(Debug)]
struct Person {
    id: u64,
    last_name: String,
}

// Custom implementation of `PartialEq` and `Eq`:
impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
        // `last_name` is ignored.
    }
}

// `Eq` has no methods.
impl Eq for Person {}

fn people() {
    let p1 = Person {
        id: 1,
        last_name: "Smith".to_string(),
    };
    let p2 = Person {
        id: 2,
        last_name: "Smith".to_string(),
    };
    assert_ne!(p1, p2);
}

fn main() {
    points();
    people();
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
