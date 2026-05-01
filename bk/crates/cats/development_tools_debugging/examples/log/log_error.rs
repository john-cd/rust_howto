#![allow(dead_code)]
// ANCHOR: example
/// Simulates executing a database query.
/// This function always returns an error for demonstration purposes.
fn execute_query(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn main() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {err}");
    }
    println!(
        "log_error example: set RUST_LOG=error to see the error log above"
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
    fn test() {
        main();
    }
}
