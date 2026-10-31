// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Example of using a single-threaded Tokio runtime.
//!
//! Tokio can run on a single thread (the `current_thread` runtime) or on a pool
//! of worker threads. A single thread is simpler and enough for programs that
//! mostly wait on I/O,.

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

    #[test]
    fn current_thread_runtime_returns_the_block_on_value()
    -> Result<(), Box<dyn std::error::Error>> {
        let value = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()?
            .block_on(async { "Hello world".len() });
        assert_eq!(value, 11);
        Ok(())
    }
}
