#![cfg(target_family = "unix")]
// ANCHOR: example
use std::env;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    // Read `config_path` from the environment variable `CONFIG`.
    // If `CONFIG` isn't set, fall back to a default config path.
    let config_path =
        env::var("CONFIG").unwrap_or("/etc/subversion/config".to_string());

    let config: String = fs::read_to_string(config_path)?;
    println!("Config: {}", config);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
