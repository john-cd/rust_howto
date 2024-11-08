#![cfg(target_family = "unix")]
// ANCHOR: example
use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::process::Command;
use std::process::Stdio;

fn main() -> Result<(), Error> {
    // NOTE: `systemd` should be installed for this example to work.
    let stdout = Command::new("journalctl")
        .stdout(Stdio::piped())
        .spawn()?
        .stdout
        .ok_or_else(|| {
            Error::new(
                ErrorKind::Other,
                "Could not capture standard
    output.",
            )
        })?;

    let reader = BufReader::new(stdout);

    reader
        .lines()
        .map_while(Result::ok)
        .filter(|line| line.contains("usb"))
        .for_each(|line| println!("{}", line));

    Ok(())
}
// ANCHOR_END: example

#[test]
#[cfg(target_family = "unix")]
fn test() -> anyhow::Result<()> {
    main()?;
    Ok(())
}
