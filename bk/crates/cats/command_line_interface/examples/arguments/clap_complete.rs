// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates generating shell completions with `clap_complete`.

use clap::Arg;
use clap::Command;
use clap::builder::ValueParser;
use clap_complete::generate_to;
use clap_complete::shells::Bash;

/// Builds the definition of the command-line interface.
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

/// Generates a Bash completion script for the command-line interface.
///
/// # Errors
/// Returns an error if the directory cannot be created or the script cannot be
/// written.
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

/// # Errors
/// Returns an error if the directory cannot be created or the script cannot be
/// written.
pub fn run() -> std::io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the clap complete example through its entry point.
    #[test]
    fn test_clap_complete_example_entry_point() -> std::io::Result<()> {
        main()
    }

    #[test]
    fn generated_bash_script_mentions_the_command_and_option()
    -> std::io::Result<()> {
        let dir = std::env::temp_dir().join("myapp-completions-test");
        std::fs::create_dir_all(&dir)?;
        let path = generate_to(Bash, &mut build_cli(), "myapp", &dir)?;
        assert_eq!(path.file_name().unwrap(), "myapp.bash");
        let script = std::fs::read_to_string(&path)?;
        assert!(script.contains("myapp"));
        assert!(script.contains("--config"));
        Ok(())
    }

    #[test]
    fn cli_definition_is_valid() {
        build_cli().debug_assert();
    }
}
