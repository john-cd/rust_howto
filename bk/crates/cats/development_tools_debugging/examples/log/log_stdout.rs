#![allow(dead_code)]
// ANCHOR: example
use env_logger::Target;

/// This example demonstrates how to configure the `env_logger` to output logs
/// to `Stdout`.
fn main() {
    // Create a new `env_logger` builder.
    env_logger::Builder::new().target(Target::Stdout).init();

    // Log an error message.
    log::error!("This error has been printed to Stdout");
    println!("log_stdout example: configured env_logger to write to stdout");
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main();
    }
}
