//! Versioned examples; compile separately with Rust 1.100 or newer.

// ANCHOR: never_type
fn unwrap_infallible(value: Result<u32, !>) -> u32 {
    let Ok(number) = value;
    number
}
// ANCHOR_END: never_type

// ANCHOR: drop_guard
fn record_operation(events: &mut Vec<&'static str>, fail: bool) -> Result<(), &'static str> {
    use std::mem::DropGuard;

    let mut guard = DropGuard::new(events, |events| events.push("cleanup"));
    guard.push("start");
    if fail {
        return Err("operation failed");
    }
    guard.push("finish");
    Ok(())
}
// ANCHOR_END: drop_guard

// ANCHOR: allocator
fn allocate_with_system() -> (Vec<u8, std::alloc::System>, Box<u8, std::alloc::System>) {
    use std::alloc::System;

    let mut bytes = Vec::new_in(System);
    bytes.push(7);
    (bytes, Box::new_in(9, System))
}
// ANCHOR_END: allocator

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn never_error_is_unreachable() {
        assert_eq!(unwrap_infallible(Ok(7)), 7);
    }

    #[test]
    fn cleanup_runs_on_normal_and_early_return() {
        let mut normal = Vec::new();
        assert!(record_operation(&mut normal, false).is_ok());
        assert_eq!(normal, ["start", "finish", "cleanup"]);

        let mut early = Vec::new();
        assert!(record_operation(&mut early, true).is_err());
        assert_eq!(early, ["start", "cleanup"]);
    }

    #[test]
    fn system_allocator_owns_both_values() {
        let (bytes, number) = allocate_with_system();
        assert_eq!(bytes, [7]);
        assert_eq!(*number, 9);
    }
}
