#![allow(dead_code)]
// ANCHOR: example
//! The `derive_more` crate extends Rust's built-in derive functionality to
//! provide more automatic implementations for common traits.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! [dependencies]
//! derive_more = "2.0.1" # Or latest
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
    x: i32,
    y: i32,
}

/// Newtype pattern with conversion traits.
#[derive(From, Into, Display, Debug)]
struct UserId(u64);

/// Struct with a constructor.
#[derive(Constructor, Debug)]
struct User {
    id: UserId,
    name: String,
    active: bool,
}

// Using `Deref` and `DerefMut`.
#[derive(Deref, DerefMut, Debug)]
struct Stack<T> {
    #[deref]
    #[deref_mut]
    items: Vec<T>,
}

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
    let user = User::new(UserId(12345), "Alice".to_string(), true);
    println!("{user:?}"); // User { id: UserId(12345), name: "Alice", active: true }.

    // Using `Deref` and `DerefMut`.
    let mut stack = Stack {
        items: vec![1, 2, 3],
    };
    stack.push(4); // Using `Vec`'s push method through `DerefMut`.
    assert_eq!(stack.len(), 4); // Using `Vec`'s len method through `Deref`.
}

// More complex example: Multiple derives on a single type.
#[derive(
    Add, AddAssign, Sub, SubAssign, From, Into, Display, Debug, Clone, Copy,
)]
struct Amount(f64);

// Custom error type with derive_more's `Error` trait.
// `std::error::Error` requires `std::fmt::Debug` and `std::fmt::Display`.
#[derive(Debug, derive_more::Error, Display, From)]
enum AppError {
    #[display("Database error")]
    DatabaseError,

    #[display("Insufficient funds")]
    InsufficientFundsError,
}

// Using multiple types with `derive_more`.
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

#[test]
fn test() {
    main();
}
// Adapted from <https://docs.rs/derive_more/latest/derive_more/>
