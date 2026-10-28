// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Polling several futures at once with `join!` and `join_all`.

use futures::join;

/// Print the input value and return it.
async fn foo(i: u32) -> u32 {
    println!("{i}");
    i
}

/// Awaits several calls of [`foo`] together with `join!` and `join_all`.
#[tokio::main]
async fn main() {
    // The `join!` macro polls multiple futures simultaneously, returning
    // a tuple of all results once complete.
    assert_eq!(join!(foo(1), foo(2)), (1, 2));
    // `join!` is variadic, so we can pass any number of futures.

    // `join_all` create a future which represents a collection of the
    // outputs of the futures given.
    let futures = vec![foo(1), foo(2), foo(3)];
    assert_eq!(futures::future::join_all(futures).await, [1, 2, 3]);
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the futures2 example through its entry point.
    #[test]
    fn test_futures2_example_entry_point() {
        main();
    }
}
