#![allow(dead_code)]
// ANCHOR: example
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

async fn do_health_check(mut hc: impl HealthCheck) {
    if !hc.check().await {
        // Use as normal.
        log_health_check_failure().await;
    } else {
        println!("Health check was normal");
    }
}

async fn do_async_op() -> bool {
    true
}

async fn log_health_check_failure() {
    println!("Health check failure");
}

#[tokio::main]
async fn main() {
    let hc = MyHealthChecker;
    do_health_check(hc).await;
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
