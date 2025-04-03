// ANCHOR: example
/// Executes a database query and logs it at the debug level.
fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

fn main() {
    // `env_logger` is a simple logger that can be configured via environment
    // variables. Example: `RUST_LOG=info ./app`
    env_logger::init();

    execute_query("DROP TABLE students");
}
// ANCHOR_END: example

use rusty_fork::rusty_fork_test;
// Runs in a separate process
rusty_fork_test! {
    #[test]
    fn test() {
        main();
    }
}
