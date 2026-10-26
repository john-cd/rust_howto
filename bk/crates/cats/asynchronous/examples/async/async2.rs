#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This is the main asynchronous function.
//! It demonstrates that calling an async function does not execute its body
//! immediately.

/// Asynchronous function that prints "world".
async fn say_world() {
    println!("world");
}

/// This is the main asynchronous function.
/// It demonstrates that calling an async function does not execute its body
/// immediately.
#[tokio::main]
async fn main() {
    // Calling `say_world()` does not execute the body of `say_world()`.
    // Futures are lazy.
    let op = say_world();

    // This `println!` comes first:
    println!("hello");

    // Awaiting `op` polls it until it completes:
    op.await;
}

// Prints:
// hello
// world
// Example from <https://tokio.rs/tokio/tutorial/hello-tokio>
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
