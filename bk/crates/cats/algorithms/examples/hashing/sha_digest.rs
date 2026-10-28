// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! SHA-256 digest of a file's contents with `ring`, printed as hexadecimal.

use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::io::Read;
use std::io::Write as _;

use anyhow::Result;
use data_encoding::HEXUPPER;
use ring::digest::Context;
use ring::digest::Digest;
use ring::digest::SHA256;

/// Calculates the SHA-256 digest of the data read from the given reader.
fn sha256_digest<R: Read>(mut reader: R) -> Result<Digest> {
    let mut context = Context::new(&SHA256);
    let mut buffer = [0; 1024];

    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        context.update(&buffer[..count]);
    }

    Ok(context.finish())
}

/// Writes a file, computes its SHA-256 digest and prints it.
fn main() -> Result<()> {
    if !fs::exists("temp")? {
        fs::create_dir("temp")?;
    }
    let path = "temp/file.txt";

    let mut output = File::create(path)?;
    write!(output, "We will generate a digest of this text")?;

    let input = File::open(path)?;
    let reader = BufReader::new(input);
    let digest = sha256_digest(reader)?;

    println!("SHA-256 digest is {}", HEXUPPER.encode(digest.as_ref()));

    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the sha digest example through its entry point.
    #[test]
    fn test_sha_digest_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
