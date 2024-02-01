use std::io::BufRead;
use std::io::BufReader;
use std::io::Error;
use std::io::ErrorKind;
use std::process::Command;
use std::process::Stdio;

fn main() -> Result<(), Error> {
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
        .filter_map(|line| line.ok())
        .filter(|line| line.contains("usb"))
        .for_each(|line| println!("{}", line));

    Ok(())
}
