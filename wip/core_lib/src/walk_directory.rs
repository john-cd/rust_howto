use std::ffi::OsString;
use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

/// Defines what files should be processed.
#[derive(Debug)]
pub struct Scope {
    /// Extensions to process.
    pub included_extensions: Vec<OsString>,
    /// File names (i.e. final component of a `Path`) to exclude.
    pub excluded_filenames: Vec<OsString>,
}

impl Default for Scope {
    fn default() -> Self {
        Self {
            included_extensions: vec![OsString::from("md")],
            excluded_filenames: vec![OsString::from("refs.incl.md"), OsString::from("SUMMARY.md")],
        }
    }
}

/// Walks through a directory and processes all files.
///
/// # Arguments
///
/// * `root` - The directory to walk through.
/// * `scope` - The scope.
/// * `f` - A function that processes one file at the time.
pub fn walk_directory_and_process_files<F>(root: &Path, scope: &Scope, f: F) -> Result<()>
where
    F: Fn(&Path) -> Result<()>,
{
    let files = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            let p = e.path();
            let file_name = p.file_name();
            let ext = p.extension();

            p.is_file()
                && file_name
                    .is_some_and(|f_n| !scope.excluded_filenames.contains(&f_n.to_os_string()))
                && ext.is_some_and(|ext| scope.included_extensions.contains(&ext.to_os_string()))
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

        // Extensions to process and excluded files.
        let scope = Scope::default();

        test_with(&test_cases, |dir_path| {
            walk_directory_and_process_files(dir_path, &scope, |_| Ok(()))?;
            Ok(())
        })?;

        Ok(())
    }
}
