#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Example of using a single-threaded Tokio runtime.

/// Example of using a single-threaded Tokio runtime.
fn main() {
    tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("the tokio2 example invariant should hold")
        .block_on(async {
            println!("Hello world");
        });
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tokio2 example through its entry point.
    #[test]
    fn test_tokio2_example_entry_point() {
        main();
    }
}
