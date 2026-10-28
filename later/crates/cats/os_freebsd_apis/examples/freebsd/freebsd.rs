// TODO Reviewed ??-??-2026.

//! Read-only queries of FreeBSD kernel state.

// ANCHOR: example
use sysctl::Ctl;
use sysctl::Sysctl;
use sysctl::SysctlError;

/// Reads and prints the kernel's operating-system type without modifying it.
///
/// # Errors
/// Propagates lookup, permission, and value-conversion errors from `sysctl`.
fn main() -> Result<(), SysctlError> {
    let os_type = Ctl::new("kern.ostype")?.value_string()?;
    println!("Kernel operating-system type: {os_type}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the FreeBSD-only sysctl query.
///
/// # Errors
/// Propagates errors reading the named kernel value.
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
