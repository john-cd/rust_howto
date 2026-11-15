// TODO Reviewed ??-??-2026.

// ANCHOR: example

//! Read-only queries of FreeBSD kernel state.
//!
//! FreeBSD is a Unix-like operating system used in servers and network
//! appliances. `sysctl` is its interface for reading kernel state, such as the
//! hostname or memory size. It runs on FreeBSD only.

use sysctl::Ctl;
use sysctl::Sysctl;
use sysctl::SysctlError;

/// Reads and prints the kernel's operating-system type without modifying it.
///
/// Reads and prints the FreeBSD kernel operating-system type through `sysctl`.
///
/// # Errors
///
/// Propagates lookup, permission, and value-conversion errors from `sysctl`.
fn main() -> Result<(), SysctlError> {
    let os_type = Ctl::new("kern.ostype")?.value_string()?;
    println!("Kernel operating-system type: {os_type}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the FreeBSD-only sysctl query.
/// Read-only queries of FreeBSD kernel state.
///
/// # Errors
///
/// Propagates errors reading the named kernel value.
/// Returns an error if a step of the example fails.
pub fn run() -> Result<(), SysctlError> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the freebsd example through its entry point.
    #[test]
    fn test_freebsd_example_entry_point() {
        main().expect("kern.ostype should be readable on FreeBSD");
    }
}
