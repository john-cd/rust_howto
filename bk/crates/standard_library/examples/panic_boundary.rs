//! Convert a synchronous callback's result into an FFI-safe status.

use std::panic::UnwindSafe;
use std::panic::{self};

// ANCHOR: example
#[repr(C)]
#[derive(Debug, PartialEq, Eq)]
pub struct BoundaryResult {
    /// 0 = success, 1 = expected failure, 2 = panic.
    status: i32,
    value: i32,
}

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

pub extern "C" fn process_number(input: i32) -> BoundaryResult {
    run_callback(|| {
        assert!(input != 0, "simulated callback defect");
        input.checked_mul(2).ok_or("overflow")
    })
}
// ANCHOR_END: example

fn main() {
    assert_eq!(process_number(3).status, 0);
}

#[cfg(test)]
mod tests {
    use super::BoundaryResult;
    use super::process_number;

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

    #[test]
    fn expected_failure() {
        assert_eq!(process_number(i32::MAX).status, 1);
    }

    #[test]
    fn panic_is_contained() {
        assert_eq!(process_number(0).status, 2);
    }
}
