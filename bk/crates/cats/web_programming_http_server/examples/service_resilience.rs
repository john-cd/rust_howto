// TODO Reviewed ??-??-2026.
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
/// Selects the simulated upstream behavior used to exercise retry decisions.
#[derive(Clone, Copy, PartialEq, Eq)]
#[cfg_attr(
    not(test),
    expect(dead_code, reason = "failure cases are exercised in the tests")
)]
enum Operation {
    /// Idempotent request that succeeds after the configured transient failures.
    Read,
    /// Non-idempotent request whose transient failure must not trigger a retry.
    Write,
    /// Request that fails immediately with a non-retryable error.
    PermanentFailure,
    /// Request that never completes and must be cancelled by the total deadline.
    Stall,
}

/// Distinguishes upstream errors from exhaustion of the caller's retry budget.
#[derive(Debug, PartialEq, Eq)]
enum Failure {
    /// Upstream failure that can be retried only for an idempotent read.
    Transient,
    /// Non-retryable upstream failure or failure to construct the example runtime.
    Permanent,
    /// All three read attempts returned a transient error.
    RetryExhausted,
    /// The total time budget elapsed while waiting for admission or a response.
    Deadline,
}

/// Calls a simulated upstream with at most three attempts and one total deadline.
///
/// Only reads are retried, preventing duplicate effects for writes. The shared
/// counter determines which read attempt first succeeds (`fail_first`) and lets
/// tests observe actual calls. Cloned services share a two-request concurrency
/// limit; this example issues attempts sequentially without backoff.
///
/// # Errors
///
/// Returns the first non-retryable failure, retry exhaustion after three failed
/// reads, or a deadline error when the timeout cancels the entire retry future.
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

/// Demonstrates a read succeeding after one transient upstream failure.
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

/// Verifies retry counts, total deadlines, and non-idempotent write handling.
#[cfg(test)]
mod tests {
    use super::*;

    /// Checks that one transient error then success.
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

    /// Checks that attempts and deadline are bounded.
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

    /// Checks that writes and permanent failures are not retried.
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
