//! Manipulate src/refs/*-refs.md files that contain all the reference
//! definitions for the book.

use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::io::Write;
use std::path::Path;

use anyhow::Result;
use itertools::Itertools;

/// Merge the contents of a text file and a `Vec` of text lines.
///
/// - Create a backup copy of the source text file.
/// - Sort and deduplicate the merged lines.
/// - Write back to the same file.
pub fn merge<P: AsRef<Path>>(filepath: P, new_lines: Vec<String>) -> Result<()> {
    let filepath = filepath.as_ref();

    let mut existing_lines: Vec<String> = vec![];
    if filepath.exists() {
        // Create a backup copy of the file...
        super::backup(filepath)?;
        let buf = BufReader::new(File::open(filepath)?);
        // Each string returned will not have a newline byte or CRLF at the end.
        existing_lines = buf.lines().collect::<Result<Vec<_>, _>>()?;
    }

    let merged_contents: Vec<String> = new_lines
        .iter()
        .chain(&existing_lines)
        // Remove end white spaces (incl. remaining newlines in new_lines).
        .map(|s| s.trim_end().to_string())
        .sorted()
        .dedup()
        .map(|s| s + "\n")
        .collect();

    write_each(filepath, &merged_contents)?;
    Ok(())
}

/// Write each item to a file.
///
/// This function takes a path and an iterator of items that can be converted to byte slices.
/// It creates a file at the given path and writes each item to the file, followed by a newline.
/// It then ensures that all data is written to the underlying storage.
///
/// `Vec<String>` and `&Vec<String>` are both `AsRef<[u8]>`.
fn write_each(
    path: impl AsRef<Path>,
    items: impl IntoIterator<Item = impl AsRef<[u8]>>,
) -> Result<()> {
    // Overwrites or creates the file.
    let mut file = File::create(path)?;
    for i in items {
        file.write_all(i.as_ref())?;
    }
    // Surface any I/O errors that could otherwise be swallowed when
    // the file is closed implicitly by being dropped.
    file.sync_all()?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read;

    use tempfile::tempdir;

    use super::*;

    // Helper to read file content.
    fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    // Write empty Vec.
    #[test]
    fn test_write_each_empty() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("empty_test.txt");
        let items: Vec<String> = vec![];

        write_each(&file_path, &items)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "");
        Ok(())
    }

    #[test]
    fn test_write_each_vec_string() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("vec_string_test.txt");
        let items = vec!["line a\n".to_string(), "line b\n".to_string()];

        write_each(&file_path, items)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "line a\nline b\n");
        Ok(())
    }

    #[test]
    fn test_write_each_ref_vec_string() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("ref_vec_string_test.txt");
        let items = vec!["ref_line1\n".to_string(), "ref_line2\n".to_string()];

        // Pass reference instead of vector.
        write_each(&file_path, &items)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "ref_line1\nref_line2\n");
        Ok(())
    }

    #[test]
    fn test_merge_new_file() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("merge_new.txt");
        let new_lines = vec!["c".to_string(), "a".to_string(), "b".to_string()];

        merge(&file_path, new_lines)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "a\nb\nc\n");

        // Check backup (should not exist as original file didn't exist).
        let backup_path = file_path.with_extension("bak");
        assert!(!backup_path.exists());

        Ok(())
    }

    #[test]
    fn test_merge_existing_file_no_new_lines() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("merge_existing_empty_new.txt");
        let initial_content = "z\nx\ny\nx\n";
        fs::write(&file_path, initial_content)?;

        let new_lines: Vec<String> = vec![];
        merge(&file_path, new_lines)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "x\ny\nz\n"); // Should sort and dedup existing

        // Check backup.
        let backup_path = file_path.with_extension("bak");
        assert!(backup_path.exists());
        let backup_content = read_file_content(&backup_path)?;
        assert_eq!(backup_content, initial_content);

        Ok(())
    }

    #[test]
    fn test_merge_existing_file_with_new_lines() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("merge_existing_with_new.txt");
        let initial_content = "c\na\ne\n";
        fs::write(&file_path, initial_content)?;

        let new_lines = vec!["b".to_string(), "d".to_string(), "a".to_string()]; // "a" overlaps.
        merge(&file_path, new_lines)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "a\nb\nc\nd\ne\n"); // Merged, sorted, deduped.

        // Check backup
        let backup_path = file_path.with_extension("bak");
        assert!(backup_path.exists());
        let backup_content = read_file_content(&backup_path)?;
        assert_eq!(backup_content, initial_content);

        Ok(())
    }

    #[test]
    fn test_merge_empty_file_with_new_lines() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("merge_empty_with_new.txt");
        fs::write(&file_path, "")?; // Create empty file

        let new_lines = vec![
            "z".to_string(),
            "y".to_string(),
            "x".to_string(),
            "y".to_string(),
        ];
        merge(&file_path, new_lines)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "x\ny\nz\n"); // Sorted and deduped new lines

        // Check backup
        let backup_path = file_path.with_extension("bak");
        assert!(backup_path.exists());
        let backup_content = read_file_content(&backup_path)?;
        assert_eq!(backup_content, ""); // Backup of empty file

        Ok(())
    }
}
