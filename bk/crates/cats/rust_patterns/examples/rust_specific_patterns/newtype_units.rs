// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Newtypes that carry a unit: only the operations that make sense are
//! implemented, so the compiler rejects mixing units by mistake.

use std::fmt;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Div;
use std::ops::Mul;
use std::ops::Sub;
use std::ops::SubAssign;

/// A length in meters.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Default)]
#[repr(transparent)]
struct Meters(f64);

impl Add for Meters {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Meters {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Sub for Meters {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Self(self.0 - rhs.0)
    }
}

impl SubAssign for Meters {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 -= rhs.0;
    }
}

/// Scaling by a plain number is fine: meters * 2.0 is still meters.
impl Mul<f64> for Meters {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self {
        Self(self.0 * rhs)
    }
}

/// Dividing two lengths gives a dimensionless ratio.
impl Div for Meters {
    type Output = f64;

    fn div(self, rhs: Self) -> f64 {
        self.0 / rhs.0
    }
}

// There is deliberately no `Mul<Meters> for Meters`: meters * meters would be
// square meters, not meters. This does not compile:
//
//     let area = Meters(2.0) * Meters(3.0);
//     // error[E0277]: cannot multiply `Meters` by `Meters`
//
// Neither does adding a bare number: `Meters(2.0) + 3.0`.

impl fmt::Display for Meters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} m", self.0)
    }
}

/// Adds, subtracts and scales lengths.
fn main() {
    let mut track = Meters(400.0);
    track += Meters(50.0);
    track -= Meters(25.0);
    let doubled = track * 2.0;

    assert_eq!(track, Meters(425.0));
    assert_eq!(doubled, Meters(850.0));
    assert_eq!(doubled / track, 2.0);
    println!("{track} doubled is {doubled}");
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that the quantity example formats its computed values.
    /// Runs the newtype units example through its entry point.
    #[test]
    fn test_newtype_units_example_entry_point() {
        main();
    }

    /// Checks the supported operations preserve their expected units.
    #[test]
    fn test_ops() {
        assert_eq!(Meters(1.0) + Meters(2.0), Meters(3.0));
        assert_eq!(Meters(3.0) - Meters(2.0), Meters(1.0));
        assert!(Meters(1.0) < Meters(2.0));
        assert_eq!(Meters::default(), Meters(0.0));
    }
}
