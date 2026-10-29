// TODO Reviewed ??-??-2026.

#![expect(
    dead_code,
    reason = "the snippet declares items to illustrate them, not to call them"
)]
// ANCHOR: example
//! The `derive_more` crate extends Rust's built-in derive functionality to
//! provide more automatic implementations for common traits.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! derive_more = { version = "2.1.1", features = ["full"] }
//! ```

use derive_more::Add;
use derive_more::AddAssign;
use derive_more::Constructor;
use derive_more::Deref;
use derive_more::DerefMut;
use derive_more::Display;
use derive_more::From;
use derive_more::Into;
use derive_more::Sub;
use derive_more::SubAssign;

/// Basic numeric type with arithmetic operations.
#[derive(Add, AddAssign, PartialEq, Debug)]
struct Point {
    /// The horizontal coordinate.
    x: i32,
    /// The vertical coordinate.
    y: i32,
}

/// Newtype pattern with conversion traits.
#[derive(From, Into, Display, Debug)]
struct UserId(u64);

/// Struct with a constructor.
#[allow(
    clippy::redundant_field_names,
    reason = "The Constructor derive generates the field initializers."
)]
#[derive(Constructor, Debug)]
struct User {
    /// The identifier of the user.
    id: UserId,
    /// The name of the user.
    name: String,
    /// Whether the user is active.
    active: bool,
}

// Using `Deref` and `DerefMut`.
/// A stack, which dereferences to its items.
#[derive(Deref, DerefMut, Debug)]
struct Stack<T> {
    /// The items of the stack.
    #[deref]
    #[deref_mut]
    items: Vec<T>,
}

/// Uses the traits derived by `derive_more`.
fn main() {
    // Using `Add` and `AddAssign`.
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let p3 = p1 + p2; // Thanks to `Add`.
    assert_eq!(p3, Point { x: 4, y: 6 });

    let mut p4 = Point { x: 5, y: 6 };
    p4 += Point { x: 1, y: 1 }; // Thanks to `AddAssign`.
    assert_eq!(p4, Point { x: 6, y: 7 });

    // Using `From` and `Into`.
    let user_id = UserId::from(12345);
    let raw_id: u64 = user_id.into();
    assert_eq!(raw_id, 12345);

    // Using `Display`.
    println!("User ID: {}", UserId(67890)); // Prints "User ID: 67890".

    // Using `Constructor`.
    let user = User::new(UserId(12345), "Alice".to_owned(), true);
    println!("{user:?}"); // User { id: UserId(12345), name: "Alice", active: true }.

    // Using `Deref` and `DerefMut`.
    let mut stack = Stack {
        items: vec![1, 2, 3],
    };
    stack.push(4); // Using `Vec`'s push method through `DerefMut`.
    assert_eq!(stack.len(), 4); // Using `Vec`'s len method through `Deref`.

    // Using the derived arithmetic and error traits.
    let mut from = Amount(10.0);
    let mut to = Amount(2.0);
    transfer_amount(&mut from, &mut to, Amount(3.0))
        .expect("the source balance covers the transfer");
    assert_eq!(from.0, 7.0);
    assert_eq!(to.0, 5.0);

    assert!(transfer_amount(&mut from, &mut to, Amount(20.0)).is_err());
    let database_error = AppError::DatabaseError;
    assert_eq!(database_error.to_string(), "Database error");
}

// More complex example: Multiple derives on a single type.
/// An amount of money.
#[derive(
    Add, AddAssign, Sub, SubAssign, From, Into, Display, Debug, Clone, Copy,
)]
struct Amount(f64);

// Custom error type with derive_more's `Error` trait.
// `std::error::Error` requires `std::fmt::Debug` and `std::fmt::Display`.
/// The errors of the application.
#[derive(Debug, derive_more::Error, Display, From)]
enum AppError {
    /// A database error.
    #[display("Database error")]
    DatabaseError,

    /// The balance is too low.
    #[display("Insufficient funds")]
    InsufficientFundsError,
}

// Using multiple types with `derive_more`.
/// Moves `value` from one amount to another, unless the balance is too low.
fn transfer_amount(
    from: &mut Amount,
    to: &mut Amount,
    value: Amount,
) -> Result<(), AppError> {
    if from.0 < value.0 {
        return Err(AppError::InsufficientFundsError);
    }

    *from -= value;
    *to += value;

    Ok(())
}
// ANCHOR_END: example

pub fn run() {
    main();
}

// Adapted from <https://docs.rs/derive_more/latest/derive_more/>

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_derive_more_example_entry_point() {
        main();
    }

    #[test]
    fn test_point_add_and_add_assign() {
        let sum = Point { x: 1, y: 2 } + Point { x: 3, y: 4 };
        assert_eq!(sum, Point { x: 4, y: 6 });
        let mut p = Point { x: 5, y: 6 };
        p += Point { x: 1, y: 1 };
        assert_eq!(p, Point { x: 6, y: 7 });
    }

    #[test]
    fn test_user_id_conversions_and_display() {
        let id = UserId::from(12345);
        assert_eq!(u64::from(id), 12345);
        assert_eq!(UserId(67890).to_string(), "67890");
    }

    #[test]
    fn test_constructor() {
        let user = User::new(UserId(1), "Alice".to_owned(), true);
        assert_eq!(user.name, "Alice");
        assert!(user.active);
        assert_eq!(
            format!("{user:?}"),
            r#"User { id: UserId(1), name: "Alice", active: true }"#
        );
    }

    #[test]
    fn test_deref_and_deref_mut_expose_vec_methods() {
        let mut stack = Stack { items: vec![1, 2] };
        stack.push(3);
        assert_eq!(stack.len(), 3);
        assert_eq!(stack.pop(), Some(3));
        assert!(!stack.is_empty());
    }

    #[test]
    fn test_transfer_amount() {
        let mut from = Amount(10.0);
        let mut to = Amount(2.0);
        transfer_amount(&mut from, &mut to, Amount(3.0)).unwrap();
        assert!((from.0 - 7.0).abs() < f64::EPSILON);
        assert!((to.0 - 5.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_transfer_amount_with_insufficient_funds_changes_nothing() {
        let mut from = Amount(1.0);
        let mut to = Amount(2.0);
        let err = transfer_amount(&mut from, &mut to, Amount(3.0)).unwrap_err();
        assert!(matches!(err, AppError::InsufficientFundsError));
        assert_eq!(err.to_string(), "Insufficient funds");
        assert!((from.0 - 1.0).abs() < f64::EPSILON);
        assert!((to.0 - 2.0).abs() < f64::EPSILON);
    }

    #[test]
    fn test_error_display() {
        assert_eq!(AppError::DatabaseError.to_string(), "Database error");
    }
}
