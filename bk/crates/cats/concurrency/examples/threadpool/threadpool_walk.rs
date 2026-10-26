#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Walk a directory and computes the SHA256 digest of all iso files.

use std::fs::File;
use std::io::BufReader;
use std::io::Error;
use std::io::Read as _;
use std::path::Path;
use std::sync::mpsc::channel;

use sha2::Digest as _;
use sha2::Sha256;
use threadpool::ThreadPool;
use walkdir::WalkDir;

/// Verifies that the path ends with an `iso` extension.
fn is_iso(entry: &Path) -> bool {
    matches!(entry.extension(), Some(e) if e.to_string_lossy().to_lowercase() == "iso")
}

/// Computes a file's SHA-256 digest and returns it with the input path.
fn compute_digest<P: AsRef<Path>>(filepath: P) -> Result<([u8; 32], P), Error> {
    let mut buf_reader = BufReader::new(File::open(&filepath)?);
    let mut hasher = Sha256::new();
    let mut buffer = [0; 1024];

    loop {
        let count = buf_reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        hasher.update(&buffer[..count]);
    }

    Ok((hasher.finalize().into(), filepath))
}

/// Walks the example directory and prints SHA-256 digests for ISO files.
///
/// # Errors
///
/// Returns an error if a file cannot be read or a worker result reports an
/// error.
fn main() -> Result<(), Error> {
    let pool = ThreadPool::new(num_cpus::get());

    let (tx, rx) = channel();

    for entry in WalkDir::new("/home/user/Downloads")
        .follow_links(true)
        .into_iter()
        .filter_map(std::result::Result::ok)
        .filter(|e| !e.path().is_dir() && is_iso(e.path()))
    {
        let path = entry.path().to_owned();
        let tx = tx.clone();
        pool.execute(move || {
            let digest = compute_digest(path);
            tx.send(digest)
                .expect("the receiver remains open until all workers finish");
        });
    }

    drop(tx);
    let mut errors = Vec::new();
    for result in rx {
        match result {
            Ok((sha, path)) => println!("{sha:02x?} {path:?}"),
            Err(error) => errors.push(error),
        }
    }
    if !errors.is_empty() {
        let messages = errors
            .into_iter()
            .map(|error| error.to_string())
            .collect::<Vec<_>>()
            .join("; ");
        return Err(Error::other(messages));
    }
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> Result<(), Error> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that walking a directory without ISO files completes
    /// successfully.
    #[test]
    fn completes_when_no_iso_files_are_present() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }
}
