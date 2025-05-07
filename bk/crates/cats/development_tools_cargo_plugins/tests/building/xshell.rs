// ANCHOR: example
//! This example demonstrates the usage of the `xshell` crate for shell
//! scripting in Rust.
//!
//! `xshell` provides a convenient way to execute shell commands, manipulate the
//! file system, and manage environment variables within a Rust program.

use xshell::Shell;
use xshell::cmd;

fn main() -> anyhow::Result<()> {
    // Create a new Shell instance.
    //
    // This provides the environment for running commands. It maintains a
    // logical working directory and an environment map. They are
    // independent from the current process's `std::env::current_dir` and
    // `std::env::var`, and only affect paths and commands passed to the
    // `Shell`.
    let sh = Shell::new()?;

    // Basic command execution:
    // The `cmd!` macro provides a convenient syntax
    // for creating a command (a `Cmd` struct).
    // `read` runs the command and return its `stdout` as a string.
    // You can also use `run`.
    let output = cmd!(sh, "echo Hello, xshell!").read()?;
    println!("Output: {}", output);

    // Run a command with arguments:
    // You don't have to worry about escaping the arguments.
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

    // Working directory manipulation:
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

    // Conditional execution of a commmand:
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

    // Check existence of the file.
    if sh.path_exists(&file_path) {
        // Read the entire contents of a file into a string.
        if let Ok(file_content) = sh.read_file(&file_path) {
            println!("File content: {}", file_content);
        }
        // Remove the file.
        sh.remove_path(file_path)?;
    }

    // Reverts the working directory to its old value when the guard is dropped.
    drop(guard);

    // Checking command status:
    let status = cmd!(sh, "true").run();
    println!("Command status: {:?}", status);

    let failed_status = cmd!(sh, "false").run();
    println!("Failed command status: {:?}", failed_status);
    // Capture `stderr` with `read_stderr`:
    let err_result = cmd!(sh, "cat nonexistent_file").read_stderr();
    println!("Standard error: {}", err_result.unwrap_err());

    // Environment variables:
    // `xshell` maintains its own environment map, independent of the system's.
    sh.set_var("MY_VAR", "my_value");
    println!("Set MY_VAR to {}", sh.var("MY_VAR")?);
    let env_var = cmd!(sh, "echo $MY_VAR").read()?;
    println!("MY_VAR environment variable: {}", env_var);

    // Change the working directory permanently:
    let temp_dir = tempfile::tempdir()?;
    let temp_path = temp_dir.path();
    sh.change_dir(temp_path);

    Ok(())
}
// ANCHOR_END: example

#[test]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
// TODO echo $MY_VAR does not work?
