// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Calls the Rust helpers used by the loadable NIF in `nif.rs`.

#[path = "nif.rs"]
mod nif;

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

/// Calls the plain Rust helpers behind the NIFs. The NIFs themselves are meant to be loaded by
/// Erlang or Elixir.
pub fn run() {
    main();
}

/// Tests behavior implemented by the surrounding module.
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
