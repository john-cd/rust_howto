// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! A plain Rust command function that a Tauri app can expose to its frontend.
//!
//! Tauri builds desktop apps with a web front end (HTML and JavaScript) and a
//! Rust back end. The front end calls Rust functions, called commands, over an
//! IPC (inter-process communication) bridge; they should validate their input
//! like any public API.

/// Greets the caller by name.
pub fn greet(name: &str) -> String {
    format!("Hello, {name}! Welcome to Tauri.")
}

/// Prints a greeting to demonstrate the Rust-side command without a Tauri
/// runtime.
pub fn main() {
    println!("{}", greet("Ferris"));
}
// ANCHOR_END: example

/// Exposes this example to the crate's dispatcher.
pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the Tauri command example entry point.
    #[test]
    fn test_tauri_application_entry_point() {
        main();
    }

    /// Verifies the command returns a greeting for the supplied name.
    #[test]
    fn greets_named_caller() {
        assert_eq!(greet("Ferris"), "Hello, Ferris! Welcome to Tauri.");
    }
}
