#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates the `frunk` crate, a functional programming
//! toolbelt for Rust.
//!
//! It covers:
//! - `HList`: statically typed heterogeneous lists.
//! - `Generic`: converting between structs and their generic (HList)
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

fn main() {
    // -----------------------------------------------------------------------
    // HList: a statically typed heterogeneous list.
    // Each element may have a different type, unlike a `Vec`.
    // -----------------------------------------------------------------------

    // Build an HList with the `hlist!` macro.
    let h = hlist![42i32, "hello", 3.14f64, true];

    // Destructure with `hlist_pat!`.
    let hlist_pat![n, s, f, b] = h;
    println!("n={n}, s={s}, f={f}, b={b}");
    assert_eq!(n, 42);
    assert_eq!(s, "hello");

    // Convert an HList into a tuple and back.
    let h2 = hlist![1u8, "world", false];
    let t: (u8, &str, bool) = h2.into();
    assert_eq!(t, (1u8, "world", false));

    let h3: frunk::HList![u8, &str, bool] = Into::into(t);
    assert_eq!(h3, hlist![1u8, "world", false]);

    // Pluck a specific type out of an HList (compile-time checked).
    // `pluck` returns the extracted value and the remaining HList.
    let h4 = hlist![1i32, "hello", true, 2.5f32];
    let (extracted, remainder): (bool, _) = h4.pluck();
    assert!(extracted);
    assert_eq!(remainder, hlist![1i32, "hello", 2.5f32]);

    // Sculpt: re-order / select a subset of types from an HList.
    let h5 = hlist![9000i32, "joe", 41.0f32, true];
    let (reshaped, remainder2): (frunk::HList![f32, i32, &str], _) =
        h5.sculpt();
    assert_eq!(reshaped, hlist![41.0f32, 9000i32, "joe"]);
    assert_eq!(remainder2, hlist![true]);

    // -----------------------------------------------------------------------
    // Generic: zero-boilerplate conversion between a struct and its HList
    // representation.
    // -----------------------------------------------------------------------

    #[derive(Generic, Debug, PartialEq)]
    struct Point {
        x: f64,
        y: f64,
    }

    // Convert an HList into a struct using `frunk::from_generic`.
    let h6 = hlist![1.0f64, 2.0f64];
    let p: Point = frunk::from_generic(h6);
    assert_eq!(p, Point { x: 1.0, y: 2.0 });

    // Convert a struct into its HList representation with `frunk::into_generic`.
    let p2 = Point { x: 3.0, y: 4.0 };
    let h7 = frunk::into_generic(p2);
    assert_eq!(h7, hlist![3.0f64, 4.0f64]);

    // Convert between two structurally identical types using `convert_from`.
    #[derive(Generic)]
    struct Point3D {
        x: f64,
        y: f64,
        z: f64,
    }

    #[derive(Generic, Debug, PartialEq)]
    struct Vec3 {
        x: f64,
        y: f64,
        z: f64,
    }

    let pt = Point3D {
        x: 1.0,
        y: 2.0,
        z: 3.0,
    };
    let v: Vec3 = frunk::convert_from(pt);
    assert_eq!(v, Vec3 { x: 1.0, y: 2.0, z: 3.0 });

    // -----------------------------------------------------------------------
    // LabelledGeneric: struct-to-struct conversion that checks field *names*.
    // If two types share the same field names (and types), they can be
    // converted without any hand-written boilerplate.
    // -----------------------------------------------------------------------

    #[derive(LabelledGeneric)]
    struct NewUser<'a> {
        first_name: &'a str,
        last_name: &'a str,
        age: usize,
    }

    #[derive(LabelledGeneric, Debug, PartialEq)]
    struct SavedUser<'a> {
        first_name: &'a str,
        last_name: &'a str,
        age: usize,
    }

    let new_user = NewUser {
        first_name: "Jane",
        last_name: "Doe",
        age: 28,
    };

    // `labelled_convert_from` only compiles when field names match.
    let saved: SavedUser = frunk::labelled_convert_from(new_user);
    assert_eq!(saved.first_name, "Jane");
    assert_eq!(saved.last_name, "Doe");
    assert_eq!(saved.age, 28);

    // `transform_from` additionally handles reordered fields via `Sculptor`.
    #[derive(LabelledGeneric, Debug, PartialEq)]
    struct ArchivedUser<'a> {
        age: usize,        // fields in a different order
        last_name: &'a str,
        first_name: &'a str,
    }

    let archived: ArchivedUser = frunk::transform_from(saved);
    assert_eq!(archived.first_name, "Jane");

    // -----------------------------------------------------------------------
    // Monoid / Semigroup: generic `combine` and `combine_all`.
    // -----------------------------------------------------------------------

    // Combine `Option<i32>` values: `None` is the identity, `Some` adds.
    let opts = vec![Some(1i32), Some(3), None, Some(6)];
    assert_eq!(combine_all(&opts), Some(10));

    // Combine tuples element-wise.
    let tuples = vec![
        (1i32, String::from("hello")),
        (2i32, String::from(", world")),
    ];
    assert_eq!(combine_all(&tuples), (3i32, String::from("hello, world")));

    println!("All frunk assertions passed!");
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
// [finish example](https://github.com/john-cd/rust_howto/issues/1318)
