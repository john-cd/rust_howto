#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! This example demonstrates the usage of the `xshell` crate for shell
//! scripting in Rust.
//!
//! `xshell` provides a convenient way to execute shell commands, manipulate the
//! file system, and manage environment variables within a Rust program.

use xshell::Shell;
use xshell::cmd;

/// Demonstrates file creation, directory navigation, and path operations.
fn demonstrate_echo_and_file_ops(sh: &Shell) -> anyhow::Result<()> {
    let output = cmd!(sh, "echo Hello, xshell!").read()?;
    println!("Output: {output}");

    let file_paths = ["test.txt"];
    let temp_dir = "temp";
    cmd!(
        sh,
        "echo We will create {file_paths...} in the {temp_dir} directory."
    )
    .run()?;

    let current_dir = sh.current_dir();
    println!("Current directory: {}", current_dir.display());

    let guard = sh.push_dir(temp_dir);
    let cwd = sh.current_dir();
    println!("New current directory: {}", cwd.display());

    sh.write_file("test.txt", "Some Content")?;

    if cmd!(sh, "test -f test.txt").quiet().run().is_ok() {
        println!("'test.txt' exists");
    }

    let file_path = sh.current_dir().join("test.txt");
    if sh.path_exists(&file_path) {
        if let Ok(file_content) = sh.read_file(&file_path) {
            println!("File content: {file_content}");
        }
        sh.remove_path(file_path)?;
    }

    drop(guard);
    Ok(())
}

/// Demonstrates environment variable management and command expansion.
fn demonstrate_env_and_expansion(sh: &Shell) -> anyhow::Result<()> {
    let status = cmd!(sh, "true").run();
    tracing::info!("Command status: {status:?}");

    let failed_status = cmd!(sh, "false").run();
    tracing::info!("Failed command status: {failed_status:?}");

    sh.set_var("MY_VAR", "my_value");
    println!("Set MY_VAR to {}", sh.var("MY_VAR")?);

    let literal = cmd!(sh, "echo $MY_VAR").read()?;
    assert_eq!(literal, "$MY_VAR");

    let my_var = sh.var("MY_VAR")?;
    let via_interpolation = cmd!(sh, "echo {my_var}").read()?;
    assert_eq!(via_interpolation, "my_value");

    let via_sh = cmd!(sh, "sh -c 'echo $MY_VAR'").read()?;
    assert_eq!(via_sh, "my_value");

    Ok(())
}

/// Runs shell commands with `xshell`.
#[tracing::instrument(err)]
fn main() -> anyhow::Result<()> {
    let _ = tracing_subscriber::fmt::try_init();
    tracing::info!("Starting xshell example");

    let sh = Shell::new()?;
    demonstrate_echo_and_file_ops(&sh)?;
    demonstrate_env_and_expansion(&sh)?;

    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();
    sh.change_dir(temp_path);

    tracing::info!("xshell example completed successfully.");
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(target_os = "linux")]
#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the xshell example through its entry point.
    #[test]
    #[cfg(target_os = "linux")]
    fn test_xshell_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    #[cfg(target_os = "linux")]
    fn test_env_and_expansion() -> anyhow::Result<()> {
        let sh = Shell::new()?;
        demonstrate_env_and_expansion(&sh)?;
        Ok(())
    }
}
