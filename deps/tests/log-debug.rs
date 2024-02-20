fn execute_query(query: &str) {
    log::debug!("Executing query: {}", query);
}

#[test]
fn test() {
    env_logger::init();

    execute_query("DROP TABLE students");
}
