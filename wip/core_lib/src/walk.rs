use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

/// Walks through a directory and processes all (Markdown) files.
///
/// # Arguments
///
/// * `root` - The directory to walk through.
/// * `f` - A function that processes one file at the time.
pub fn walk_directory_and_process_files<F>(root: &Path, f: F) -> Result<()>
where
    F: Fn(&Path) -> Result<()>,
{
    let files = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
            // TODO generalize
                && e.path().extension().is_some_and(|ext| ext == "md")
                && !e.path().ends_with("refs.incl.md")
                && !e.path().ends_with("SUMMARY.md")
            // && !e.path().to_string_lossy().contains("refs.md")
        })
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    for file in files {
        tracing::info!("Processing {}.", file.display());
        f(&file)?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {

    use anyhow::Result;

    use super::*;
    use crate::TestCase;
    use crate::test_with;

    // Test the null case: no modifications to files.
    #[test]
    fn test_walk_directory() -> Result<()> {
        let test_cases = vec![
            // Markdown file.
            TestCase {
                file_name_and_ext: "test1.md",
                original_contents: Some("test1 this\n"),
                expected_final_contents: Some("test1 this\n"),
            },
            // Other file (excluded).
            TestCase {
                file_name_and_ext: "test2.txt",
                original_contents: Some("\n"),
                expected_final_contents: Some("\n"),
            },
            // Empty markdown file.
            TestCase {
                file_name_and_ext: "test3.md",
                original_contents: None,
                expected_final_contents: None,
            },
        ];

        test_with(&test_cases, |dir_path| {
            walk_directory_and_process_files(dir_path, |_| Ok(()))?;
            Ok(())
        })?;

        Ok(())
    }
}
