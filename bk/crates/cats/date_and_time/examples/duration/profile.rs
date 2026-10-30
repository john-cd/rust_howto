// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates how to profile the execution time of a function
//! using `Instant` and `Duration`.
//!
//! It defines an `expensive_function` that simulates a time-consuming operation
//! by sleeping for 1 second. The `main` function measures the time taken by
//! `expensive_function` and prints the duration.
//!
//! `Instant` is a monotonic clock: it only moves forward and is not affected
//! by changes of the system clock, which makes it the right tool for measuring
//! elapsed time. (Use `SystemTime` for calendar time.) For serious benchmarks,
//! prefer a tool such as `criterion`, which repeats the measurement.

use std::thread;
use std::time::Duration;
use std::time::Instant;

/// Sleeps for one second to simulate an expensive computation.
fn expensive_function() {
    thread::sleep(Duration::from_secs(1));
}

/// Measures the time spent in [`expensive_function`].
fn main() {
    let start = Instant::now();
    expensive_function();
    let duration = start.elapsed();

    println!("Time elapsed in expensive_function() is: {duration:?}");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Confirms that a measured duration is nonnegative without asserting its
    /// value.
    #[test]
    fn elapsed_duration_is_nonnegative() {
        let start = Instant::now();
        let duration = start.elapsed();
        assert!(duration >= Duration::ZERO);
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
