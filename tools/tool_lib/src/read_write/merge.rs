//! Manipulate src/refs/*-refs.md files that contain all the reference
//! definitions for the book.
//!
//! TODO Reviewed ??-??-2026.

use std::fs::File;
use std::io::BufRead as _;
use std::io::BufReader;
use std::path::Path;

use anyhow::Result;
use itertools_014::Itertools as _;

/// Merge the contents of a text file and a `Vec` of text lines.
///
/// - Create a backup copy of the source text file.
/// - Sort and deduplicate the merged lines.
/// - Write back to the same file by atomic replacement.
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
        .map(|s| s.trim_end().to_owned())
        .sorted()
        .dedup()
        .map(|s| s + "\n")
        .collect();

    write_each(filepath, &merged_contents)?;
    Ok(())
}

/// Write each item to a file, replacing it atomically.
///
/// The items are written back to back (callers include the newlines) to a
/// temporary sibling that replaces `path` only after a complete, synced write.
///
/// `Vec<String>` and `&Vec<String>` are both `AsRef<[u8]>`.
fn write_each(
    path: impl AsRef<Path>,
    items: impl IntoIterator<Item = impl AsRef<[u8]>>,
) -> Result<()> {
    super::atomic::write_atomic(path, items)
}

#[cfg(test)]
mod tests {
    use std::fs;
    use std::io::Read as _;

    use tempfile::tempdir;

    use super::*;

    // Helper to read file content.
    /// Reads UTF-8 text from `path`.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be opened or read.
    fn read_file_content<P: AsRef<Path>>(path: P) -> Result<String> {
        let mut file = File::open(path)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        Ok(content)
    }

    // Write empty Vec.
    /// Verifies that write each empty.
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

    /// Verifies that write each vec string.
    #[test]
    fn test_write_each_vec_string() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("vec_string_test.txt");
        let items = vec!["line a\n".to_owned(), "line b\n".to_owned()];

        write_each(&file_path, items)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "line a\nline b\n");
        Ok(())
    }

    /// Verifies that write each ref vec string.
    #[test]
    fn test_write_each_ref_vec_string() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("ref_vec_string_test.txt");
        let items = vec!["ref_line1\n".to_owned(), "ref_line2\n".to_owned()];

        // Pass reference instead of vector.
        write_each(&file_path, &items)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "ref_line1\nref_line2\n");
        Ok(())
    }

    /// Verifies that merge new file.
    #[test]
    fn test_merge_new_file() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("merge_new.txt");
        let new_lines = vec!["c".to_owned(), "a".to_owned(), "b".to_owned()];

        merge(&file_path, new_lines)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "a\nb\nc\n");

        // Check backup (should not exist as original file didn't exist).
        let backup_path = super::super::backup::backup_path(&file_path);
        assert!(!backup_path.exists());

        Ok(())
    }

    /// Verifies that merge existing file no new lines.
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
        let backup_path = super::super::backup::backup_path(&file_path);
        assert!(backup_path.exists());
        let backup_content = read_file_content(&backup_path)?;
        assert_eq!(backup_content, initial_content);

        Ok(())
    }

    /// Verifies that merge existing file with new lines.
    #[test]
    fn test_merge_existing_file_with_new_lines() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("merge_existing_with_new.txt");
        let initial_content = "c\na\ne\n";
        fs::write(&file_path, initial_content)?;

        let new_lines = vec!["b".to_owned(), "d".to_owned(), "a".to_owned()]; // "a" overlaps.
        merge(&file_path, new_lines)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "a\nb\nc\nd\ne\n"); // Merged, sorted, deduped.

        // Check backup
        let backup_path = super::super::backup::backup_path(&file_path);
        assert!(backup_path.exists());
        let backup_content = read_file_content(&backup_path)?;
        assert_eq!(backup_content, initial_content);

        Ok(())
    }

    /// Verifies that merge empty file with new lines.
    #[test]
    fn test_merge_empty_file_with_new_lines() -> Result<()> {
        let dir = tempdir()?;
        let file_path = dir.path().join("merge_empty_with_new.txt");
        fs::write(&file_path, "")?; // Create empty file

        let new_lines = vec![
            "z".to_owned(),
            "y".to_owned(),
            "x".to_owned(),
            "y".to_owned(),
        ];
        merge(&file_path, new_lines)?;

        let content = read_file_content(&file_path)?;
        assert_eq!(content, "x\ny\nz\n"); // Sorted and deduped new lines

        // Check backup
        let backup_path = super::super::backup::backup_path(&file_path);
        assert!(backup_path.exists());
        let backup_content = read_file_content(&backup_path)?;
        assert_eq!(backup_content, ""); // Backup of empty file

        Ok(())
    }
}
