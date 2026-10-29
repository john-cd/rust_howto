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
fn exit_program() -> ! {
    println!("Exiting the process...");
    std::process::exit(1);
}

// This function loops forever.
/// Loops forever.
fn forever() -> ! {
    loop {
        println!("and ever...");
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}

/// Calls a function that panics.
fn main() {
    // Diverging functions can be referenced as function pointers, even if
    // calling them would end the process or never finish:
    let diverging: [fn() -> !; 2] = [exit_program, forever];
    println!(
        "{} diverging functions that are not called.",
        diverging.len()
    );

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
