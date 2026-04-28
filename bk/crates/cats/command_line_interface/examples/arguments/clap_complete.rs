#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates generating shell completions with `clap_complete`.

use std::path::PathBuf;

use clap::Arg;
use clap::Command;
use clap_complete::generate_to;
use clap_complete::shells::Bash;

fn build_cli() -> Command {
    Command::new("myapp").about("A simple CLI application").arg(
        Arg::new("config")
            .short('c')
            .long("config")
            .help("Path to the config file")
            .takes_value(true),
    )
}

fn main() -> std::io::Result<()> {
    // Build the CLI application definition.
    let mut cmd = build_cli();

    // Use the system temporary directory for generated completion files.
    let out_dir = std::env::temp_dir().join("myapp-completions");

    // Generate Bash completion scripts into the output directory.
    let out_dir = generate_to(Bash, &mut cmd, "myapp", &out_dir)?;
    println!(
        "Generated Bash completion files into: {}",
        out_dir.display()
    );
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test() {
        main().unwrap();
    }
}
// TODO add to a chapter on command line interfaces with clap_complete
