#![allow(dead_code)]
// ANCHOR: example
//! This example demonstrates generating shell completions with `clap_complete`.

use clap::Arg;
use clap::Command;
use clap::builder::ValueParser;
use clap_complete::generate_to;
use clap_complete::shells::Bash;

fn build_cli() -> Command {
    Command::new("myapp").about("A simple CLI application").arg(
        Arg::new("config")
            .short('c')
            .long("config")
            .help("Path to the config file")
            .num_args(1)
            .value_parser(ValueParser::path_buf()),
    )
}

fn main() -> std::io::Result<()> {
    // Build the CLI application definition.
    let mut cmd = build_cli();

    // Use the system temporary directory for generated completion files.
    let out_dir = std::env::temp_dir().join("myapp-completions");
    std::fs::create_dir_all(&out_dir)?;

    // Generate Bash completion scripts into the output directory.
    let out_dir = generate_to(Bash, &mut cmd, "myapp", &out_dir)?;
    println!(
        "Generated Bash completion files into: {}",
        out_dir.display()
    );
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> std::io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_main() {
        main().unwrap();
    }
}
// TODO add to a chapter on command line interfaces with clap_complete
