// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Converting between types with `From` and `Into`.
//!
//! `From` defines a lossless conversion between types, and gives you `Into` for
//! free. Implementing `From` for your error types is what lets `?` convert
//! errors automatically.

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

    #[test]
    fn test_from_example_entry_point() {
        main();
    }

    #[test]
    fn test_from_and_into_are_equivalent() {
        let via_into: Millimeters = Meters(2).into();
        let via_from = Millimeters::from(Meters(2));
        assert_eq!(via_into.0, 2000);
        assert_eq!(via_from.0, via_into.0);
    }

    #[test]
    fn test_zero_meters() {
        assert_eq!(Millimeters::from(Meters(0)).0, 0);
    }

    #[test]
    fn test_debug_output() {
        assert_eq!(
            format!("{:?}", Millimeters::from(Meters(1))),
            "Millimeters(1000)"
        );
    }
}
