// TODO Reviewed ??-??-2026
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
    fn greeting(name: String) -> String {
        format!("Hello, {name}!")
    }

    #[rustler::nif]
    fn greet(name: String) -> String {
        greeting(name)
    }

    /// `add` is a NIF that takes two integers and returns their sum.
    fn sum(a: i32, b: i32) -> i32 {
        a + b
    }

    #[rustler::nif]
    fn add(a: i32, b: i32) -> i32 {
        sum(a, b)
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
