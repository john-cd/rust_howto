#![allow(dead_code)]
// ANCHOR: example
/// Executes a database query and logs it at the debug level.
fn execute_query(query: &str) {
    log::debug!("Executing query: {query}");
}

fn main() {
    // `env_logger` is a simple logger that can be configured via environment
    // variables. Example: `RUST_LOG=info ./app`.
    env_logger::init();

    execute_query("DROP TABLE students");
    println!(
        "log_debug example: set RUST_LOG=debug to see the debug log above"
    );
}

// ANCHOR_END: example

pub fn run() {
    main();
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main();
    }
}
