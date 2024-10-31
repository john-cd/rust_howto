use std::env;

fn env_extract() -> String {
    let log_env_var = env::var("RUST_LOG").unwrap_or_else(|_| "debug".into());
    println!("RUST_LOG: {log_env_var}");

    let user_env_var = env::var("USER").expect("$USER is not set");
    println!("USER: {user_env_var}");

    // Inspect an environment variable at compile-time.
    // Uncomment to test.
    // let shell = env!("SHELL", "$SHELL is not set");

    let optional_value = option_env!("SHELL");

    optional_value.unwrap_or("no shell set").to_string()
}

fn main() {
    println!("SHELL: {}", env_extract());
}

#[should_panic]
#[test]
fn test() {
    main();
}
