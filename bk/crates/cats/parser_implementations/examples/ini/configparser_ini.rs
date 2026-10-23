// TODO Reviewed ??-??-2026
// ANCHOR: example
//! This example demonstrates parsing INI text and using typed getters with
//! `configparser`.

use anyhow::Context;
use anyhow::anyhow;
use configparser::ini::Ini;

/// Parses an INI configuration string and prints its host, port, and serialized
/// form.
fn main() -> anyhow::Result<()> {
    let input = "[server]\nhost = 127.0.0.1\nport = 8080\n";
    let mut config = Ini::new();
    config
        .read(input.to_owned())
        .map_err(anyhow::Error::msg)
        .context("parse server configuration")?;

    let host = config
        .get("SERVER", "HOST")
        .ok_or_else(|| anyhow!("missing [server] host"))?;
    let port = config
        .getuint("server", "port")
        .map_err(anyhow::Error::msg)?
        .ok_or_else(|| anyhow!("missing [server] port"))?;

    println!("Listening on {host}:{port}");

    let _previous_value = config.setstr("server", "host", Some("0.0.0.0"));
    println!("Updated configuration:\n{}", config.writes());
    Ok(())
}

// ANCHOR_END: example

/// Runs the `configparser` example.
pub fn run() -> anyhow::Result<()> {
    main()
}
