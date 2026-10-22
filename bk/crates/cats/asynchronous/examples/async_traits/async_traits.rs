//! An `async fn` in a trait, implemented by a health checker.
//!
//! TODO Reviewed ??-??-2026

#![allow(dead_code)]

// ANCHOR: example
/// A health checker that implements [`HealthCheck`].
struct MyHealthChecker;

/// A trait for checking the health of a system.
trait HealthCheck {
    /// Checks the health of the system.
    ///
    /// Returns `true` if the system is healthy, `false` otherwise.
    async fn check(&mut self) -> bool;
}

impl HealthCheck for MyHealthChecker {
    // `async fn` implementation in the associated `impl` block.
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

/// Runs a health check on the Tokio runtime.
#[tokio::main]
async fn main() {
    let hc = MyHealthChecker;
    do_health_check(hc).await;
}
// ANCHOR_END: example

/// Runs the example.
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
