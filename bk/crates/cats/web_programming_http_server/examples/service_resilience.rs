//! A bounded, in-memory Tower client with a total deadline.

use std::future::pending;
use std::sync::Arc;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;
use std::time::Duration;

use tower::ServiceBuilder;
use tower::ServiceExt;
use tower::service_fn;

// ANCHOR: example
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "failure cases are exercised in the tests")
)]
enum Operation {
    Read,
    Write,
    PermanentFailure,
    Stall,
}

#[derive(Debug, PartialEq, Eq)]
enum Failure {
    Transient,
    Permanent,
    RetryExhausted,
    Deadline,
}

async fn call_with_budget(
    operation: Operation,
    fail_first: usize,
    calls: Arc<AtomicUsize>,
    budget: Duration,
) -> Result<u8, Failure> {
    let upstream = service_fn(move |request: Operation| {
        let calls = Arc::clone(&calls);
        async move {
            let attempt = calls.fetch_add(1, Ordering::SeqCst);
            match request {
                Operation::Read if attempt >= fail_first => Ok(42),
                Operation::Read | Operation::Write => Err(Failure::Transient),
                Operation::PermanentFailure => Err(Failure::Permanent),
                Operation::Stall => pending::<Result<u8, Failure>>().await,
            }
        }
    });
    let limited = ServiceBuilder::new().concurrency_limit(2).service(upstream);
    let retry = async {
        for attempt in 0..3 {
            match limited.clone().oneshot(operation).await {
                Ok(value) => return Ok(value),
                Err(Failure::Transient) if operation == Operation::Read && attempt < 2 => {}
                Err(Failure::Transient) if operation == Operation::Read => {
                    return Err(Failure::RetryExhausted);
                }
                Err(other) => return Err(other),
            }
        }
        unreachable!("the last attempt returns");
    };
    tokio::time::timeout(budget, retry)
        .await
        .map_err(|_| Failure::Deadline)?
}
// ANCHOR_END: example

fn main() -> Result<(), Failure> {
    let runtime = tokio::runtime::Builder::new_current_thread()
        .build()
        .map_err(|_| Failure::Permanent)?;
    let calls = Arc::new(AtomicUsize::new(0));
    let result = runtime.block_on(call_with_budget(
        Operation::Read,
        1,
        calls,
        Duration::from_secs(1),
    ))?;
    assert_eq!(result, 42);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test(start_paused = true)]
    async fn one_transient_error_then_success() {
        let calls = Arc::new(AtomicUsize::new(0));
        assert_eq!(
            call_with_budget(
                Operation::Read,
                1,
                Arc::clone(&calls),
                Duration::from_secs(1)
            )
            .await,
            Ok(42)
        );
        assert_eq!(calls.load(Ordering::SeqCst), 2);
    }

    #[tokio::test(start_paused = true)]
    async fn attempts_and_deadline_are_bounded() {
        let calls = Arc::new(AtomicUsize::new(0));
        assert_eq!(
            call_with_budget(
                Operation::Read,
                9,
                Arc::clone(&calls),
                Duration::from_secs(1)
            )
            .await,
            Err(Failure::RetryExhausted)
        );
        assert_eq!(calls.load(Ordering::SeqCst), 3);

        let calls = Arc::new(AtomicUsize::new(0));
        assert_eq!(
            call_with_budget(Operation::Stall, 0, calls, Duration::from_millis(5)).await,
            Err(Failure::Deadline)
        );
    }

    #[tokio::test(start_paused = true)]
    async fn writes_and_permanent_failures_are_not_retried() {
        for (operation, expected) in [
            (Operation::Write, Failure::Transient),
            (Operation::PermanentFailure, Failure::Permanent),
        ] {
            let calls = Arc::new(AtomicUsize::new(0));
            assert_eq!(
                call_with_budget(operation, 0, Arc::clone(&calls), Duration::from_secs(1)).await,
                Err(expected)
            );
            assert_eq!(calls.load(Ordering::SeqCst), 1);
        }
    }
}
