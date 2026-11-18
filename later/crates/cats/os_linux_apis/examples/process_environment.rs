// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Starts an owned helper with explicit arguments, directory and environment.

use std::ffi::OsStr;
use std::io;
use std::path::Path;
use std::process::Command;
use std::process::Stdio;

/// Configures a trusted executable without invoking a shell or changing parent state.
fn helper_command(executable: &Path, directory: &Path) -> Command {
    let mut command = Command::new(executable);
    command
        .current_dir(directory)
        .env_clear()
        .env("BOOK_MODE", "fixture");
    // Windows system components can require this even with an absolute executable.
    #[cfg(windows)]
    if let Some(system_root) = std::env::var_os("SystemRoot") {
        command.env("SystemRoot", system_root);
    }
    command
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    command
}

/// Runs a fixed-output child and reports a nonzero exit as an error.
fn run_helper(executable: &Path, directory: &Path, fail: bool) -> io::Result<String> {
    let mut command = helper_command(executable, directory);
    command.arg("--child");
    if fail {
        command.arg("--fail");
    }
    let output = command.output()?;
    if !output.status.success() {
        return Err(io::Error::other(format!(
            "helper exited with {}",
            output.status
        )));
    }
    String::from_utf8(output.stdout)
        .map_err(|error| io::Error::new(io::ErrorKind::InvalidData, error))
}

/// Runs a fixed child protocol, or starts that protocol in the current executable.
fn main() -> io::Result<()> {
    let args: Vec<_> = std::env::args_os().skip(1).collect();
    if args.first().is_some_and(|arg| arg == OsStr::new("--child")) {
        if std::env::var_os("BOOK_MODE").as_deref() != Some(OsStr::new("fixture")) {
            return Err(io::Error::other("missing helper environment"));
        }
        if args.get(1).is_some_and(|arg| arg == OsStr::new("--fail")) {
            return Err(io::Error::other("requested failure"));
        }
        println!("helper ready");
        return Ok(());
    }
    let executable = std::env::current_exe()?;
    let directory = tempfile::tempdir()?;
    let text = run_helper(&executable, directory.path(), false)?;
    assert_eq!(text, "helper ready\n");
    assert!(run_helper(&executable, directory.path(), true).is_err());
    print!("{text}");
    directory.close()
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Arguments containing spaces and shell punctuation stay single literal arguments.
    #[test]
    fn command_has_explicit_environment_and_arguments() -> io::Result<()> {
        let executable = std::env::current_exe()?;
        let directory = tempfile::tempdir()?;
        let mut command = helper_command(&executable, directory.path());
        command.arg("a file; echo ignored");
        assert_eq!(
            command.get_args().collect::<Vec<_>>(),
            vec![OsStr::new("a file; echo ignored")]
        );
        assert_eq!(command.get_current_dir(), Some(directory.path()));
        assert!(
            command
                .get_envs()
                .any(|(key, value)| key == "BOOK_MODE" && value == Some(OsStr::new("fixture")))
        );
        directory.close()
    }

    /// Failure to launch an executable remains a distinct spawn error.
    #[test]
    fn missing_program_is_reported() -> io::Result<()> {
        let directory = tempfile::tempdir()?;
        assert!(
            run_helper(
                &directory.path().join("not-a-program"),
                directory.path(),
                false
            )
            .is_err()
        );
        directory.close()
    }
}
