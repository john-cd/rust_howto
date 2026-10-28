#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates the `frunk` crate, a functional programming
//! toolbelt for Rust.
//!
//! It covers:
//! - `HList`: statically typed heterogeneous lists.
//! - `Generic`: converting between structs and their generic (`HList`)
//!   representations.
//! - `LabelledGeneric`: field-name-safe struct-to-struct conversions.
//! - `Monoid`/`Semigroup`: combining values generically.
//!
//! Add to your `Cargo.toml`:
//! ```toml
//! frunk = "0.4.4"
//! frunk_core = "0.4.4"
//! ```

use frunk::Generic;
use frunk::LabelledGeneric;
use frunk::hlist;
use frunk::hlist_pat;
use frunk::monoid::combine_all;

/// A heterogeneous list of three elements.
type Sculpted = frunk::HList![f32, i32, &'static str];

/// Demonstrates heterogeneous lists.
fn demonstrate_hlist() {
    let h = hlist![42i32, "hello", std::f64::consts::PI, true];

    let hlist_pat![n, s, f, b] = h;
    println!("n={n}, s={s}, f={f}, b={b}");
    assert_eq!(n, 42);
    assert_eq!(s, "hello");

    let h2 = hlist![1u8, "world", false];
    let t: (u8, &str, bool) = h2.into();
    assert_eq!(t, (1u8, "world", false));

    let h3: frunk::HList![u8, &str, bool] = Into::into(t);
    assert_eq!(h3, hlist![1u8, "world", false]);

    let h4 = hlist![1i32, "hello", true, 2.5f32];
    let (extracted, remainder): (bool, _) = h4.pluck();
    assert!(extracted);
    assert_eq!(remainder, hlist![1i32, "hello", 2.5f32]);

    let h5 = hlist![9000i32, "joe", 41.0f32, true];
    let (reshaped, remainder2): (Sculpted, _) = h5.sculpt();
    assert_eq!(reshaped, hlist![41.0f32, 9000i32, "joe"]);
    assert_eq!(remainder2, hlist![true]);
}

/// Represents a two-dimensional point.
#[derive(Generic, Debug, PartialEq)]
struct Point {
    /// The horizontal coordinate.
    x: f64,
    /// The vertical coordinate.
    y: f64,
}

/// Represents a three-dimensional point.
#[derive(Generic)]
struct Point3D {
    /// The horizontal coordinate.
    x: f64,
    /// The vertical coordinate.
    y: f64,
    /// The depth coordinate.
    z: f64,
}

/// Represents a three-dimensional vector.
#[derive(Generic, Debug, PartialEq)]
struct Vec3 {
    /// The horizontal coordinate.
    x: f64,
    /// The vertical coordinate.
    y: f64,
    /// The depth coordinate.
    z: f64,
}

/// Demonstrates generic conversion between structs and HLists.
fn demonstrate_generic() {
    let h6 = hlist![1.0f64, 2.0f64];
    let p: Point = frunk::from_generic(h6);
    assert_eq!(p, Point { x: 1.0, y: 2.0 });

    let p2 = Point { x: 3.0, y: 4.0 };
    let h7 = frunk::into_generic(p2);
    assert_eq!(h7, hlist![3.0f64, 4.0f64]);

    let pt = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v: Vec3 = frunk::convert_from(pt);
    assert_eq!(
        v,
        Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0
        }
    );
}

/// Represents a user before it is saved.
#[derive(LabelledGeneric)]
struct NewUser<'a> {
    /// The first name.
    first_name: &'a str,
    /// The last name.
    last_name: &'a str,
    /// Age in years.
    age: usize,
}

/// Represents a user after it is saved.
#[derive(LabelledGeneric, Debug, PartialEq)]
struct SavedUser<'a> {
    /// The first name.
    first_name: &'a str,
    /// The last name.
    last_name: &'a str,
    /// Age in years.
    age: usize,
}

/// Represents an archived user record.
#[derive(LabelledGeneric, Debug, PartialEq)]
struct ArchivedUser<'a> {
    /// Age in years.
    age: usize,
    /// The last name.
    last_name: &'a str,
    /// The first name.
    first_name: &'a str,
}

/// Demonstrates LabelledGeneric conversion.
fn demonstrate_labelled_generic() {
    let new_user = NewUser {
        first_name: "Jane",
        last_name: "Doe",
        age: 28,
    };

    let saved: SavedUser = frunk::labelled_convert_from(new_user);
    assert_eq!(saved.first_name, "Jane");
    assert_eq!(saved.last_name, "Doe");
    assert_eq!(saved.age, 28);

    let archived: ArchivedUser = frunk::transform_from(saved);
    assert_eq!(archived.first_name, "Jane");
}

/// Demonstrates monoids and generic combination.
fn demonstrate_monoid() {
    let opts = vec![Some(1i32), Some(3), None, Some(6)];
    assert_eq!(combine_all(&opts), Some(10));

    let tuples = vec![
        (1i32, String::from("hello")),
        (2i32, String::from(", world")),
    ];
    assert_eq!(combine_all(&tuples), (3i32, String::from("hello, world")));
}

/// Demonstrates heterogeneous lists, labelled generics and monoids.
fn main() {
    demonstrate_hlist();
    demonstrate_generic();
    demonstrate_labelled_generic();
    demonstrate_monoid();
    println!("All frunk assertions passed!");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the frunk example through its entry point.
    #[test]
    fn test_frunk_example_entry_point() {
        main();
    }

    #[test]
    fn test_demonstrate_hlist() {
        demonstrate_hlist();
    }

    #[test]
    fn test_demonstrate_generic() {
        demonstrate_generic();
    }

    #[test]
    fn test_demonstrate_labelled_generic() {
        demonstrate_labelled_generic();
    }

    #[test]
    fn test_demonstrate_monoid() {
        demonstrate_monoid();
        let empty_opts: Vec<Option<i32>> = vec![];
        assert_eq!(combine_all(&empty_opts), None);
    }
}
