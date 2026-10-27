#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Converting between types with `From` and `Into`.

/// A length in millimeters.
#[derive(Debug)]
struct Millimeters(u32);

/// A length in meters.
#[derive(Debug)]
struct Meters(u32);

impl From<Meters> for Millimeters {
    fn from(m: Meters) -> Self {
        Self(m.0 * 1000)
    }
}

/// Converts meters to millimeters with `from` and `into`.
fn main() {
    let m = Meters(2);
    let mm: Millimeters = m.into();
    println!("{mm:?}");
    assert_eq!(mm.0, 2000);
    // OR:
    let m = Meters(4);
    let mm = Millimeters::from(m);
    println!("{mm:?}");
    assert_eq!(mm.0, 4000);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the from example through its entry point.
    #[test]
    fn test_from_example_entry_point() {
        main();
    }
}
