fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

fn main() {
    env_logger::init();

    execute_query("DROP TABLE students");
}

use rusty_fork::rusty_fork_test;

rusty_fork_test! {
#[test]
fn test() {
    main();
}
}
