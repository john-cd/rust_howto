// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! This example demonstrates the usage of the `xshell` crate for shell
//! scripting in Rust.
//!
//! `xshell` provides a convenient way to execute shell commands, manipulate the
//! file system, and manage environment variables within a Rust program.

use xshell::Shell;
use xshell::cmd;

/// Runs shell commands with `xshell`.
#[tracing::instrument(err)]
fn main() -> anyhow::Result<()> {
    // Initialize tracing unless the test harness already installed a
    // subscriber.
    let _ = tracing_subscriber::fmt::try_init();
    tracing::info!("Starting xshell example");

    // Create a new Shell instance.
    //
    // This provides the environment for running commands. It maintains a
    // logical working directory and an environment map. They are
    // independent from the current process's `std::env::current_dir` and
    // `std::env::var`, and only affect paths and commands passed to the
    // `Shell`.
    let sh = Shell::new()?;

    // Run a command and capture its standard output.
    // The `cmd!` macro provides a convenient syntax
    // for creating a command (a `Cmd` struct).
    // `read` runs the command and returns its `stdout` as a string.
    // You can also use `run`.
    let output = cmd!(sh, "echo Hello, xshell!").read()?;
    println!("Output: {output}");

    // Pass command arguments separately so `xshell` can handle their quoting.
    let file_paths = ["test.txt"];
    let temp_dir = "temp";
    cmd!(
        sh,
        "echo We will create {file_paths...} in the {temp_dir} directory."
    )
    .run()?;
    // Note how the so-called splat syntax `...` is used to interpolate an
    // iterable of arguments. This is a convenient way to pass multiple
    // or optional arguments to a command.

    // Inspect and temporarily change the shell's working directory.
    let current_dir = sh.current_dir();
    println!("Current directory: {}", current_dir.display());

    // Temporarily changes the working directory of this Shell.
    //
    // Returns a RAII guard which reverts the working directory to the old value
    // when dropped.
    let guard = sh.push_dir(temp_dir);
    let cwd = sh.current_dir();
    println!("New current directory: {}", cwd.display());

    // Write to a file. Paths are relative to the current directory of the
    // Shell. Creates the file and all intermediate directories
    // if they don't exist.
    sh.write_file("test.txt", "Some Content")?;

    // Check for a file and branch on the command's exit status.
    if cmd!(sh, "test -f test.txt").quiet().run().is_ok() {
        println!("'test.txt' exists");
    } else {
        println!("'test.txt' does not exist");
    }
    // `quiet()` prevents echoing the command itself to `stderr`.
    // This is useful when you don't want to see the command in the output.

    // Path manipulation.
    let file_path = sh.current_dir().join("text.txt");
    println!("File path: {}", file_path.display());

    // Check whether the generated file exists before reading or removing it.
    if sh.path_exists(&file_path) {
        // Read the entire contents of a file into a string.
        if let Ok(file_content) = sh.read_file(&file_path) {
            println!("File content: {file_content}");
        }
        // Remove the file.
        sh.remove_path(file_path)?;
    }

    // Reverts the working directory to its old value when the guard is dropped.
    drop(guard);

    // Inspect successful and failing command statuses.
    let status = cmd!(sh, "true").run();
    tracing::info!("Command status: {status:?}");

    let failed_status = cmd!(sh, "false").run();
    tracing::info!("Failed command status: {failed_status:?}");
    // Capture a failed command's standard error with `read_stderr`.
    let err_result = cmd!(sh, "cat nonexistent_file").read_stderr();
    tracing::info!("Standard error: {}", err_result.unwrap_err());

    // Set and read variables in the shell's environment map.
    // `xshell` maintains its own environment map, independent of the system's.
    sh.set_var("MY_VAR", "my_value");
    println!("Set MY_VAR to {}", sh.var("MY_VAR")?);

    // Important: `xshell` does NOT invoke a shell, so shell variable expansion
    // does NOT happen. `$MY_VAR` is passed literally to the command.
    // This command prints `$MY_VAR` literally because `xshell` does not expand
    // shell variables.
    let literal = cmd!(sh, "echo $MY_VAR").read()?;
    println!("Direct echo $MY_VAR (literal, not expanded): '{literal}'");
    assert_eq!(literal, "$MY_VAR");

    // Read the value with `sh.var()` and interpolate it as a Rust variable in
    // the `cmd!` macro.
    let my_var = sh.var("MY_VAR")?;
    let via_interpolation = cmd!(sh, "echo {my_var}").read()?;
    println!("Via Rust variable interpolation: '{via_interpolation}'");
    assert_eq!(via_interpolation, "my_value");

    // Alternatively, to expand shell variables you can invoke a shell
    // explicitly. Note: this is NOT cross-platform (requires `sh` to be
    // available).
    let via_sh = cmd!(sh, "sh -c 'echo $MY_VAR'").read()?;
    println!("Via sh -c (shell expansion): '{via_sh}'");
    assert_eq!(via_sh, "my_value");

    // Change the working directory permanently for this shell.
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
}
