// ANCHOR: example
use std::env;

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
