#![allow(dead_code)]
// ANCHOR: example
use std::env;

/// Extracts the value of the `RUST_LOG` environment variable.
/// If the variable is not set, it defaults to "debug".
/// It also demonstrates how to handle optional environment variables
/// and how to inspect them at compile-time.
fn env_extract() -> String {
    let log_env_var = env::var("RUST_LOG").unwrap_or_else(|_| "debug".into());
    println!("RUST_LOG: {log_env_var}");

    // You may also choose to panic if the env. variable is not set:
    // Uncomment to test.
    // let user_env_var = env::var("USER").expect("$USER is not set");
    // println!("USER: {user_env_var}");

    // Inspect an environment variable at compile-time.
    // let shell = env!("SHELL", "$SHELL is not set");

    let optional_value = option_env!("SHELL");

    optional_value.unwrap_or("no shell set").to_string()
}

fn main() {
    println!("SHELL: {}", env_extract());
}
// ANCHOR_END: example

#[test]
fn test() {
    main();
}
