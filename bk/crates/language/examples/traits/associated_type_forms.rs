//! Demonstrates associated types and the default right-hand operand of Add.
//!
//! TODO Reviewed ??-??-2026.

// ANCHOR: associated_types
use std::fmt::Debug;
use std::fmt::Display;

/// Defines the operations required for tr behavior.
trait Tr {
    /// Type of the item values produced or consumed by this trait.
    type Item;
    // Associated type with a trait bound.
    type Item2: Display;
    // Associated type with multiple trait bounds.
    type Item3: Debug + Display;
    // Generic associated type.
    type Item4<T>;
    // Lifetime, type, and const parameters can be combined.
    type Item5<'a, T: 'a, const N: usize>;
    // An associated type can constrain its type parameter.
    type Item6<T>
    where
        T: Clone;
}
// ANCHOR_END: associated_types

struct Example;

impl Tr for Example {
    /// Alias for `u8` used to represent item.
    type Item = u8;
    /// Alias for `String` used to represent item2.
    type Item2 = String;
    /// Alias for `i32` used to represent item3.
    type Item3 = i32;
    /// Alias for `Option<T>` used to represent item4.
    type Item4<T> = Option<T>;
    /// Alias for `&'a [T; N]` used to represent item5.
    type Item5<'a, T: 'a, const N: usize> = &'a [T; N];
    /// Alias for `Vec<T>` used to represent item6.
    type Item6<T: Clone> = Vec<T>;
}

/// Projecting each associated type yields a value of the type chosen in the implementation.
#[test]
fn uses_the_implementation_types() {
    let item: <Example as Tr>::Item = 7;
    let display: <Example as Tr>::Item2 = String::from("display");
    let both: <Example as Tr>::Item3 = -7;
    let generic: <Example as Tr>::Item4<u8> = Some(item);
    let array: <Example as Tr>::Item5<'_, u8, 2> = &[1, 2];
    let owned: <Example as Tr>::Item6<u8> = array.to_vec();

    assert_eq!(
        (display.as_str(), both, generic, owned),
        ("display", -7, Some(7), vec![1, 2])
    );
}

// ANCHOR: add
/// A simplified model of core::ops::Add.
trait Add<Rhs = Self> {
    /// The type of the result.
    type Output;

    /// Adds two values together.
    fn add(self, rhs: Rhs) -> Self::Output;
}
// ANCHOR_END: add

struct Value(
    /// Numeric value stored by the Value wrapper.
    u32,
);

impl Add for Value {
    /// Alias for `u32` used to represent output.
    type Output = u32;

    /// Adds the supplied value to the current collection or state.
    fn add(self, rhs: Self) -> Self::Output {
        self.0 + rhs.0
    }
}

/// Omitting the right-hand type uses Self and still allows a different result type.
#[test]
fn defaults_the_right_hand_operand_to_self() {
    assert_eq!(Value(2).add(Value(3)), 5);
}
