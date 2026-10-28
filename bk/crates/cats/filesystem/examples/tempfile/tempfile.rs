#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Creating temporary files and directories with `tempfile`.

use std::fs::File;
use std::io::Read;
use std::io::Seek as _;
use std::io::SeekFrom;
use std::io::Write as _;

use tempfile::Builder;
use tempfile::NamedTempFile;
use tempfile::TempDir;
use tempfile::spooled_tempfile;
use tempfile::tempdir;
use tempfile::tempfile;

/// Demonstrates creating and reading an unnamed temporary file.
fn create_unnamed_tempfile() -> std::io::Result<()> {
    let mut temp_file: File = tempfile()?;
    writeln!(temp_file, "This is temporary.")?;
    temp_file.seek(SeekFrom::Start(0))?;

    let mut buf = String::new();
    temp_file.read_to_string(&mut buf)?;
    assert_eq!(buf, "This is temporary.\n");
    Ok(())
}

/// Demonstrates spooled temporary files that roll to disk when exceeding a size limit.
fn create_spooled_tempfile(threshold: usize) -> std::io::Result<bool> {
    let mut file = spooled_tempfile(threshold);
    writeln!(file, "short")?;
    let initial_rolled = file.is_rolled();
    writeln!(file, "much longer than the max size")?;
    let final_rolled = file.is_rolled();
    Ok(!initial_rolled && final_rolled)
}

/// Demonstrates creating and closing a temporary directory.
fn create_temp_directory() -> std::io::Result<()> {
    let temp_dir: TempDir = tempdir()?;
    let temp_file_path = temp_dir.path().join("temporary_file.txt");
    let mut temp_file = File::create(&temp_file_path)?;
    writeln!(temp_file, "This is also temporary.")?;
    drop(temp_file);
    temp_dir.close()?;
    Ok(())
}

/// Demonstrates creating and reopening a named temporary file.
fn create_named_tempfile() -> std::io::Result<String> {
    let mut temp_file = Builder::new()
        .prefix("my-temporary-file")
        .suffix(".txt")
        .rand_bytes(6)
        .tempfile()?;

    writeln!(temp_file, "Hello, named temporary file!")?;

    let mut file = temp_file.reopen()?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}

/// Creates temporary files and directories.
fn main() -> std::io::Result<()> {
    create_unnamed_tempfile()?;
    assert!(create_spooled_tempfile(15)?);
    create_temp_directory()?;
    let content = create_named_tempfile()?;
    assert!(content.contains("Hello, named temporary file!"));
    Ok(())
}

// ANCHOR_END: example

pub fn run() -> std::io::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the tempfile example through its entry point.
    #[test]
    fn test_tempfile_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    #[test]
    fn test_unnamed_tempfile() -> std::io::Result<()> {
        create_unnamed_tempfile()
    }

    #[test]
    fn test_spooled_tempfile() -> std::io::Result<()> {
        assert!(create_spooled_tempfile(10)?);
        Ok(())
    }

    #[test]
    fn test_temp_directory() -> std::io::Result<()> {
        create_temp_directory()
    }

    #[test]
    fn test_named_tempfile() -> std::io::Result<()> {
        let content = create_named_tempfile()?;
        assert!(content.contains("Hello, named temporary file!"));
        Ok(())
    }
}
