// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! An `async fn` in a trait, implemented by a health checker.
//!
//! Since Rust 1.75, traits can declare `async fn` directly. A trait lets
//! different types provide the same behaviour, such as several kinds of health
//! check, behind one interface.

/// A health checker that implements [`HealthCheck`].
struct MyHealthChecker;

/// A trait for checking the health of a system.
trait HealthCheck {
    /// Returns `true` if the system is healthy, `false` otherwise.
    async fn check(&mut self) -> bool;
}

impl HealthCheck for MyHealthChecker {
    // `async fn` implementation in the associated `impl` block.
    /// Checks the current value.
    async fn check(&mut self) -> bool {
        do_async_op().await
    }
}
/// Checks the health of `hc` and logs a failure if it is unhealthy.
async fn do_health_check(mut hc: impl HealthCheck) {
    if hc.check().await {
        println!("Health check was normal");
    } else {
        // Use as normal.
        log_health_check_failure().await;
    }
}

/// Simulates an asynchronous operation and returns whether it succeeded.
async fn do_async_op() -> bool {
    true
}
/// Logs a failed health check.
async fn log_health_check_failure() {
    println!("Health check failure");
}
/// Runs the example.
///
/// Runs a health check on the Tokio runtime.
#[tokio::main]
async fn main() {
    let hc = MyHealthChecker;
    do_health_check(hc).await;
}
// ANCHOR_END: example

/// Runs the example entry point.
///
/// Runs the example.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the async traits example through its entry point.
    #[test]
    fn test_async_traits_example_entry_point() {
        main();
    }

    /// Returns `true` when the built-in health checker reports healthy.
    #[tokio::test]
    async fn health_checker_reports_healthy() {
        assert!(MyHealthChecker.check().await);
    }

    /// Confirms that a failing implementation is handled by the check routine.
    #[tokio::test]
    async fn health_check_runs_with_a_failing_checker() {
        /// Groups the data required by async traits operations.
        struct Failing;
        impl HealthCheck for Failing {
            /// Checks the current value.
            async fn check(&mut self) -> bool {
                false
            }
        }
        assert!(!Failing.check().await);
        do_health_check(Failing).await;
        do_health_check(MyHealthChecker).await;
    }
}
