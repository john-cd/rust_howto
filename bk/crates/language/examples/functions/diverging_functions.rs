// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Functions that never return.

/// This function diverges, meaning it never returns.
/// It uses the `!` (Never) return type to indicate this.
fn foo() -> ! {
    panic!("This call never returns.");
}

// This function never returns control.
/// Exits the process with an error code.
#[expect(dead_code, reason = "declared to illustrate it; never used")]
fn exit_program() -> ! {
    println!("Exiting the process...");
    std::process::exit(1);
}

// This function loops forever.
/// Loops forever.
#[expect(dead_code, reason = "declared to illustrate it; never used")]
fn forever() -> ! {
    loop {
        println!("and ever...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

/// Calls a function that panics.
fn main() {
    foo(); // This will panic.
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[should_panic]
    /// Runs the diverging functions example through its entry point.
    #[test]
    fn test_diverging_functions_example_entry_point() {
        main();
    }

    /// `foo` never returns: it panics with its message.
    #[test]
    #[should_panic(expected = "This call never returns.")]
    fn test_foo_panics() {
        foo();
    }
}
