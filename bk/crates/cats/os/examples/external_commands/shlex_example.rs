// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Split and quote command lines with `shlex`.
//!
//! Shells split a command line into words using quoting rules. `shlex` applies
//! the same rules, so you can parse a command typed by a user, or quote a value
//! safely when you build a command line.

use std::process::Command;

/// Formats a program and its arguments as a line that can be pasted into a
/// POSIX shell. Use it for display or logs only.
fn display_command(
    program: &str,
    args: &[&str],
) -> Result<String, shlex::QuoteError> {
    let mut parts = vec![shlex::try_quote(program)?.into_owned()];
    for arg in args {
        parts.push(shlex::try_quote(arg)?.into_owned());
    }
    Ok(parts.join(" "))
}

/// Splits a command string and quotes arguments.
pub fn run() -> anyhow::Result<()> {
    // Split a string the way a POSIX shell would, honoring quotes.
    let words = shlex::split(r#"git commit -m "fix: handle 'quotes'""#)
        .ok_or_else(|| anyhow::anyhow!("unbalanced quotes"))?;
    assert_eq!(words, ["git", "commit", "-m", "fix: handle 'quotes'"]);

    // Quote arguments that contain spaces or special characters.
    let line = display_command("echo", &["hello world", "it's"])?;
    println!("{line}");
    // The result splits back into the original words.
    assert_eq!(
        shlex::split(&line),
        Some(vec!["echo".into(), "hello world".into(), "it's".into()])
    );

    // Quoting is for display. To run a command, pass each argument to
    // `Command::args` instead of building a string for a shell.
    if let Some((program, args)) = words.split_first() {
        let command = {
            let mut command = Command::new(program);
            command.args(args);
            command
        };
        println!("would run {:?}", command.get_program());
    }

    // NUL bytes cannot be quoted.
    assert!(shlex::try_quote("bad\0arg").is_err());
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()
    }
}
