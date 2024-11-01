// ANCHOR: example
fn execute_query(_query: &str) -> Result<(), &'static str> {
    Err("I'm afraid I can't do that")
}

fn main() {
    env_logger::init();

    let response = execute_query("DROP TABLE students");
    if let Err(err) = response {
        log::error!("Failed to execute query: {}", err);
    }
}

use rusty_fork::rusty_fork_test;
// Runs in a separate process
rusty_fork_test! {

// ANCHOR_END: example
#[test]
fn test() {
    main();
}
}
