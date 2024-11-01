// ANCHOR: example
struct MyHealthChecker;

trait HealthCheck {
    async fn check(&mut self) -> bool; // <- async fn defined in a Trait
}

impl HealthCheck for MyHealthChecker {
    async fn check(&mut self) -> bool {
        // async fn implementation in the associated impl block
        do_async_op().await
    }
}

async fn do_health_check(mut hc: impl HealthCheck) {
    if !hc.check().await {
        // use as normal
        log_health_check_failure().await;
    }
}

async fn do_async_op() -> bool {
    true
}

async fn log_health_check_failure() {}

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
