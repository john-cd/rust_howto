// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Native implemented functions for Erlang and Elixir with `rustler`.

#[cfg(feature = "erlang")]
use rustler::Env;
#[cfg(feature = "erlang")]
use rustler::NifResult;

/// The functions exposed to Erlang and Elixir.
#[expect(
    dead_code,
    reason = "These functions are exported as NIFs for the Erlang runtime."
)]
mod nif {
    #[cfg(feature = "erlang")]
    use super::*;

    /// The `rustler` crate provides macros and functions for creating NIFs
    /// (Native Implemented Functions). NIFs are functions written in Rust that
    /// can be called directly from Erlang or Elixir code.
    fn greeting(name: String) -> String {
        format!("Hello, {name}!")
    }

    /// Returns a greeting for `name`.
    #[cfg_attr(feature = "erlang", rustler::nif)]
    fn greet(name: String) -> String {
        greeting(name)
    }

    /// `add` is a NIF that takes two integers and returns their sum.
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Returns the sum of `a` and `b`.
    #[cfg_attr(feature = "erlang", rustler::nif)]
    fn add(a: i32, b: i32) -> i32 {
        sum(a, b)
    }

    #[cfg(test)]
    mod tests {
        use super::greeting;
        use super::sum;

        /// Checks that the greeting contains the name.
        #[test]
        fn greet_returns_a_personalized_message() {
            assert_eq!(greeting("Rust".to_owned()), "Hello, Rust!");
        }

        /// Checks that the sum is correct.
        #[test]
        fn add_returns_the_sum() {
            assert_eq!(sum(20, 22), 42);
        }
    }

    #[cfg(feature = "erlang")]
    rustler::init!("Elixir.MyRustLibrary");
}

/// Prints a message: the functions are meant to be loaded by Erlang or Elixir.
fn main() {
    #[cfg(not(feature = "erlang"))]
    println!("Rustler example requires the `erlang` feature.");
    #[cfg(feature = "erlang")]
    println!(
        "Rustler NIFs defined. These are intended to be loaded by an Elixir/Erlang VM."
    );
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the rustler example through its entry point.
    #[test]
    fn test_rustler_example_entry_point() {
        main();
    }
}
// [review; how to test](https://github.com/john-cd/rust_howto/issues/1080)
