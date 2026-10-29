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
    use std::io::Cursor;

    use super::*;

    const EMPTY_SHA256: &str =
        "E3B0C44298FC1C149AFBF4C8996FB92427AE41E4649B934CA495991B7852B855";

    #[test]
    fn test_sha_digest_example_entry_point() {
        main().expect("the example should succeed");
        let contents = fs::read_to_string("temp/file.txt")
            .expect("the example writes the file");
        assert_eq!(contents, "We will generate a digest of this text");
    }

    #[test]
    fn digest_of_abc_is_the_known_value() {
        let digest = sha256_digest(Cursor::new(b"abc"))
            .expect("reading from memory cannot fail");
        assert_eq!(
            HEXUPPER.encode(digest.as_ref()),
            "BA7816BF8F01CFEA414140DE5DAE2223B00361A396177A9CB410FF61F20015AD"
        );
    }

    #[test]
    fn digest_of_empty_input_is_the_known_value() {
        let digest = sha256_digest(Cursor::new(b""))
            .expect("reading from memory cannot fail");
        assert_eq!(HEXUPPER.encode(digest.as_ref()), EMPTY_SHA256);
    }

    #[test]
    fn digest_spanning_several_buffers_matches_one_shot() {
        let data = vec![b'x'; 5000];
        let streamed = sha256_digest(Cursor::new(&data))
            .expect("reading from memory cannot fail");
        let one_shot = ring::digest::digest(&SHA256, &data);
        assert_eq!(streamed.as_ref(), one_shot.as_ref());
    }

    #[test]
    fn read_errors_are_propagated() {
        struct Failing;
        impl Read for Failing {
            fn read(&mut self, _buf: &mut [u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("boom"))
            }
        }
        assert!(sha256_digest(Failing).is_err());
    }
}
