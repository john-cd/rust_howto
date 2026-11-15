// TODO Reviewed ??-??-2026.

// ANCHOR: example

//!

#![cfg_attr(
    feature = "erlang",
    allow(
        clippy::mem_forget,
        reason = "Rustler's init macro keeps its generated function table alive for the VM lifetime"
    )
)]

//!

/// The functions exposed to Erlang and Elixir.
mod nif {

    /// The `rustler` crate provides macros and functions for creating NIFs
    /// (Native Implemented Functions). NIFs are functions written in Rust that
    /// can be called directly from Erlang or Elixir code.
    ///
    /// # Arguments
///
    /// * `name` - Human-readable value used to identify or display the selected rustler.
    pub fn greeting(name: String) -> String {
        format!("Hello, {name}!")
    }

    /// Returns a greeting for `name`.
    ///
    /// # Arguments
///
    /// * `name` - Human-readable value used to identify or display the selected rustler.
    #[cfg(feature = "erlang")]
    #[rustler::nif]
    fn greet(name: String) -> String {
        greeting(name)
    }

    /// Adds two integers for calls from Erlang or Elixir.
///
    /// # Arguments
///
    /// `add` is a NIF that takes two integers and returns their sum.
    pub fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    /// Returns the sum of `a` and `b`.
    #[cfg(feature = "erlang")]
    #[rustler::nif]
    fn add(a: i32, b: i32) -> i32 {
        sum(a, b)
    }

    #[cfg(test)]
    mod tests {
        use super::greeting;
        use super::sum;

        /// Returns a personalized message for the greet.
        #[test]
        fn greet_returns_a_personalized_message() {
            assert_eq!(greeting("Rust".to_owned()), "Hello, Rust!");
        }

        /// Returns the sum for the add.
        #[test]
        fn add_returns_the_sum() {
            assert_eq!(sum(20, 22), 42);
        }
    }

    #[cfg(feature = "erlang")]
    rustler::init!("Elixir.MyRustLibrary");
}

/// Calls the plain Rust helpers behind the NIFs. The NIFs themselves are meant
/// to be loaded by Erlang or Elixir.
fn main() {
    println!("{}", nif::greeting("Rust".to_owned()));
    println!("20 + 22 = {}", nif::sum(20, 22));
    #[cfg(not(feature = "erlang"))]
    println!("Rustler example requires the `erlang` feature.");
    #[cfg(feature = "erlang")]
    println!("Rustler NIFs defined. These are intended to be loaded by an Elixir/Erlang VM.");
}
// ANCHOR_END: example

/// Calls the plain Rust helpers behind the NIFs. The NIFs themselves are meant to be loaded by Erlang or Elixir.
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
