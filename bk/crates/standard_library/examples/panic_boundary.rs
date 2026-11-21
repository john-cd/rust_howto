// TODO Reviewed ??-??-2026.
//! Convert a synchronous callback's result into an FFI-safe status.

use std::panic::UnwindSafe;
use std::panic::{self};

// ANCHOR: example
/// C-compatible callback outcome with an explicit status instead of Rust unwinding.
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct BoundaryResult {
    /// 0 = success, 1 = expected failure, 2 = panic.
    status: i32,
    /// Doubled input on success; zero when the callback fails or panics.
    value: i32,
}

/// Converts callback success, an expected error, or an unwinding panic to a status.
///
/// Logs error details locally while keeping the C-facing result allocation-free.
/// `catch_unwind` only contains unwinding panics; panic-abort builds still abort.
fn run_callback<F>(callback: F) -> BoundaryResult
where
    F: FnOnce() -> Result<i32, &'static str> + UnwindSafe,
{
    match panic::catch_unwind(callback) {
        Ok(Ok(value)) => BoundaryResult { status: 0, value },
        Ok(Err(message)) => {
            eprintln!("callback failed: {message}");
            BoundaryResult {
                status: 1,
                value: 0,
            }
        }
        Err(payload) => {
            let message = payload
                .downcast_ref::<&str>()
                .copied()
                .or_else(|| payload.downcast_ref::<String>().map(String::as_str))
                .unwrap_or("non-string panic payload");
            eprintln!("callback panicked: {message}");
            BoundaryResult {
                status: 2,
                value: 0,
            }
        }
    }
}

/// Doubles the input behind a panic boundary for callers using the C ABI.
///
/// Returns status 1 on arithmetic overflow and status 2 for the deliberately
/// panicking zero-input example; successful results use status 0.
pub extern "C" fn process_number(input: i32) -> BoundaryResult {
    run_callback(|| {
        assert!(input != 0, "simulated callback defect");
        input.checked_mul(2).ok_or("overflow")
    })
}
// ANCHOR_END: example

/// Demonstrates a successful call through the C-compatible entrypoint.
fn main() {
    assert_eq!(process_number(3).status, 0);
}

/// Checks all three callback statuses exposed to foreign callers.
#[cfg(test)]
mod tests {
    use super::BoundaryResult;
    use super::process_number;

    /// Returns the doubled value with the success status.
    #[test]
    fn success() {
        assert_eq!(
            process_number(3),
            BoundaryResult {
                status: 0,
                value: 6
            }
        );
    }

    /// Reports arithmetic overflow as an expected callback failure.
    #[test]
    fn expected_failure() {
        assert_eq!(process_number(i32::MAX).status, 1);
    }

    /// Converts the deliberate zero-input panic into a failure status.
    #[test]
    fn panic_is_contained() {
        assert_eq!(process_number(0).status, 2);
    }
}
