#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates the usage of the `comemo` crate for fine-grained
//! memoization.
//!
//! Memoization speeds up execution by storing the results of
//! expensive function calls and returning the cached result when the same
//! inputs occur again.
//!
//! `comemo` improves on basic memoization by tracking dependencies.
//! For example, when a memoized function accesses a tracked value, `comemo`
//! records the access. If the tracked value changes later, the memoized
//! function's cached result is invalidated, and the computation is re-executed.
//! This approach avoids unnecessary recomputation and optimizes performance.
//!
//! This is particularly useful for incremental computation or
//! compilation, where fine-grained access tracking is required.

use comemo::Track;
use comemo::Tracked;
use comemo::memoize;
use comemo::track;

/// Memoized function to compute a derived value based on the configuration.
///
/// Note the `#[memoize]` attribute.
///
/// We also wrapped the `config` argument in comemo's `Tracked` container.
/// It automatically track all config accesses during a memoized call.
#[memoize]
fn compute_derived_value(config: Tracked<Config>) -> i32 {
    // The only methods accessible on `Tracked<T>` are those defined in an
    // implementation block or trait for T annotated with `#[track]`. See below.
    let base_value = config.get_value();

    println!("`compute_derived_value` called; base_value = {base_value}");
    // In real-life, this would be an expensive computation.
    base_value * 2 + 10
}

/// Define a struct to represent a configuration.
#[derive(Clone, Debug)]
struct Config {
    value: i32,
    a_bool: bool,
}

/// `#[track]` makes a type trackable.
/// This attribute can be applied to an inherent implementation block or trait
/// definition. It implements the `Track` trait for the type or trait object.
/// Tracked impl blocks or traits may not be generic and may only contain
/// methods, among other [restrictions](https://docs.rs/comemo/0.4.0/comemo/attr.track.html).
#[track]
impl Config {
    fn get_value(&self) -> i32 {
        self.value
    }
}

fn main() {
    let mut config = Config {
        value: 5,
        a_bool: false,
    };

    // Note the call to `track`.
    let result = compute_derived_value(config.track());
    println!("First Call - Cache Miss. Result: {result}");

    println!(
        "Second Call - Cache Hit. We called the memoized function with the exact same argument. Result: {}",
        compute_derived_value(config.track())
    );

    config.a_bool = true;
    println!(
        "Third Call - Cache hit: `config` has changed, but the tracked value has not. Result: {}",
        compute_derived_value(config.track())
    );

    config = Config {
        value: 6,
        a_bool: false,
    };
    println!(
        "Fourth Call - Cache Miss: the tracked value has changed. Result: {}",
        compute_derived_value(config.track())
    );
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main();
    Ok(())
}
