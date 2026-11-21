#![cfg_attr(
    feature = "erlang",
    allow(
        clippy::mem_forget,
        reason = "Rustler's init macro keeps its generated function table alive for the VM lifetime"
    )
)]
//! Rustler NIF functions for Erlang and Elixir.
//!
//! TODO Reviewed ??-??-2026.
//!
//! ANCHOR: example

/// The functions exposed to Erlang and Elixir.
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

#[cfg(test)]
mod tests {
    use super::greeting;
    use super::sum;

    #[test]
    fn greet_returns_a_personalized_message() {
        assert_eq!(greeting("Rust".to_owned()), "Hello, Rust!");
    }

    #[test]
    fn add_returns_the_sum() {
        assert_eq!(sum(20, 22), 42);
    }
}

/// Returns the sum of `a` and `b`.
#[cfg(feature = "erlang")]
#[rustler::nif]
fn add(a: i32, b: i32) -> i32 {
    sum(a, b)
}

#[cfg(feature = "erlang")]
rustler::init!("Elixir.MyRustLibrary");

// ANCHOR_END: example
