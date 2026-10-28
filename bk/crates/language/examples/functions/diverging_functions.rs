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
#[allow(dead_code)]
fn exit_program() -> ! {
    println!("Exiting the process...");
    std::process::exit(1);
}

// This function loops forever.
/// Loops forever.
#[allow(dead_code)]
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
}
