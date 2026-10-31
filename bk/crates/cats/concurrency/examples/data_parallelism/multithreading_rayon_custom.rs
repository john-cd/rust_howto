// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Running `rayon` work on a custom thread pool.
//!
//! Rayon runs parallel iterators on a global thread pool, which defaults to one
//! thread per CPU core. Building a custom pool lets you choose the thread count
//! or isolate a workload.

/// Computes a Fibonacci number on a pool of eight threads.
fn main() {
    // Build the threadpool:
    let pool = rayon::ThreadPoolBuilder::new()
        .num_threads(8)
        .build()
        .expect(
            "the multithreading rayon custom example invariant should hold",
        );
    // `install` executes the closure within the threadpool. Any attempts
    // to use join, scope, or parallel iterators will then operate
    // within that threadpool.
    let n = pool.install(|| fib(20));
    println!("{n}");
}

/// Calculates the nth fibonacci number:
fn fib(n: usize) -> usize {
    if n == 0 || n == 1 {
        return n;
    }
    // Conceptually, calling `join()` is similar to spawning two threads,
    // one executing each of the two closures.
    let (a, b) = rayon::join(|| fib(n - 1), || fib(n - 2)); // runs inside of `pool`.
    a + b
}
// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the multithreading rayon custom example through its entry point.
    #[test]
    fn test_multithreading_rayon_custom_example_entry_point() {
        main();
    }

    #[test]
    fn fib_matches_known_values() {
        let known = [0, 1, 1, 2, 3, 5, 8, 13, 21, 34, 55];
        for (n, expected) in known.into_iter().enumerate() {
            assert_eq!(fib(n), expected);
        }
        assert_eq!(fib(20), 6765);
    }

    #[test]
    fn custom_pool_has_the_requested_number_of_threads()
    -> Result<(), Box<dyn std::error::Error>> {
        let pool = rayon::ThreadPoolBuilder::new().num_threads(3).build()?;
        assert_eq!(pool.current_num_threads(), 3);
        assert_eq!(pool.install(|| fib(10)), 55);
        Ok(())
    }
}
