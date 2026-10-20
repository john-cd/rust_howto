#![allow(dead_code)]
// ANCHOR: example
#![allow(unused)]

#[cfg(feature = "erlang")]
use rustler::Env;
#[cfg(feature = "erlang")]
use rustler::NifResult;

#[cfg(feature = "erlang")]
mod nif {
    use super::*;

    /// The `rustler` crate provides macros and functions for creating NIFs
    /// (Native Implemented Functions). NIFs are functions written in Rust that
    /// can be called directly from Erlang or Elixir code.
    #[rustler::nif]
    fn greet(name: String) -> String {
        format!("Hello, {name}!")
    }

    /// `add` is a NIF that takes two integers and returns their sum.
    #[rustler::nif]
    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    rustler::init!("Elixir.MyRustLibrary");
}

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
    #[test]
    fn test_main() {
        main();
    }
}
// [review; how to test](https://github.com/john-cd/rust_howto/issues/1080)
