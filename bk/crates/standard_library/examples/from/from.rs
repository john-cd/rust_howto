#![allow(dead_code)]
// ANCHOR: example
#[derive(Debug)]
struct Millimeters(u32);

#[derive(Debug)]
struct Meters(u32);

impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self {
        Millimeters(m.0 * 1000)
    }
}

fn main() {
    let m = Meters(2);
    let mm: Millimeters = m.into();
    println!("{mm:?}");
    assert_eq!(mm.0, 2000);
    // OR
    let mm = Millimeters::from(m);
    println!("{mm:?}");
    assert_eq!(mm.0, 2000);
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
