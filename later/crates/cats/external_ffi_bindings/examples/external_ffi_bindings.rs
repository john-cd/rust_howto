// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compare a raw Unix process-ID binding with the standard library.

/// Reads the process ID through the C library on Unix.
#[cfg(unix)]
fn native_process_id() -> libc::pid_t {
    // SAFETY: getpid takes no arguments and has no caller preconditions on
    // Unix.
    unsafe { libc::getpid() }
}

/// Prints the process ID, comparing the native binding on Unix.
fn main() {
    let process_id = std::process::id();
    #[cfg(unix)]
    assert_eq!(i64::from(native_process_id()), i64::from(process_id));
    println!("Process ID: {process_id}");
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the external ffi bindings example through its entry point.
    #[test]
    fn test_external_ffi_bindings_example_entry_point() {
        main();
    }

    /// Verifies the native Unix binding identifies this process.
    #[cfg(unix)]
    #[test]
    fn native_process_id_matches_standard_library() {
        assert_eq!(
            i64::from(native_process_id()),
            i64::from(std::process::id())
        );
    }
}
