// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Marks the code to profile with the `profiling` facade.
//!
//! `profiling` forwards each scope to a backend chosen with a Cargo feature,
//! for example `profile-with-puffin`. With no backend feature, the macros
//! expand to nothing and cost nothing.

/// Adds the squares of the numbers up to `n`.
///
/// `#[profiling::function]` creates a scope named after the function.
#[profiling::function]
fn sum_of_squares(n: u64) -> u64 {
    (1..=n).map(|i| i * i).sum()
}

/// Runs the work of one "frame": one iteration of a game or UI loop,
/// or one request of a server.
fn run_frame() -> u64 {
    // A named scope that lasts until the end of the enclosing block.
    profiling::scope!("run_frame");

    let total = {
        // A scope with extra data: the second argument is shown next to the
        // name.
        profiling::scope!("small", "n = 100");
        sum_of_squares(100)
    };
    let big = {
        profiling::scope!("big", "n = 10000");
        sum_of_squares(10_000)
    };
    // Tell the backend that the frame is complete.
    profiling::finish_frame!();
    total + big
}

/// Runs a few frames.
fn main() {
    let mut checksum = 0;
    for _ in 0..3 {
        checksum += run_frame();
    }
    assert_eq!(checksum, 3 * (338_350 + 333_383_335_000));
    println!("checksum: {checksum}");
}
// ANCHOR_END: example

pub fn run() {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the scoped work with profiling macros disabled or enabled.
    /// Runs the profiling scopes example through its entry point.
    #[test]
    fn test_profiling_scopes_example_entry_point() {
        main();
    }

    /// Records one frame with `puffin` and counts the scopes.
    #[test]
    fn scopes_exist_only_with_a_backend() {
        let view = puffin::GlobalFrameView::default();
        puffin::set_scopes_on(true);
        run_frame();
        puffin::GlobalProfiler::lock().new_frame();

        let frame = view.lock().latest_frame();
        let scopes = frame.map_or(0, |frame| frame.meta().num_scopes);
        if cfg!(feature = "profile-with-puffin") {
            // At least run_frame, small, big and two calls to sum_of_squares.
            // Other tests may add scopes to the same global frame.
            assert!(scopes >= 5);
        } else {
            assert_eq!(scopes, 0);
        }
    }
}
