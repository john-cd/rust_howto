use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

pub mod replace;

/// Walks through a directory and processes all markdown files.
///
/// # Arguments
///
/// * `root` - The directory to walk through.
/// * `f` - Function that processes one file at the time.
pub fn walk_directory<F>(root: &Path, f: F) -> Result<()>
where
    F: Fn(&Path) -> Result<()>,
{
    let files = WalkDir::new(root)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| {
            e.file_type().is_file()
                && e.path().extension().is_some_and(|ext| ext == "md")
                // TODO && !e.path().ends_with("refs.incl.md")
                && !e.path().ends_with("SUMMARY.md")
            // && !e.path().to_string_lossy().contains("refs.md")
        })
        .map(|e| e.path().to_path_buf())
        .collect::<Vec<_>>();

    for file in files {
        println!("Processing {}.", file.display());
        f(&file)?;
    }
    Ok(())
}

// TODO finish

#[cfg(test)]
mod tests {
    use std::fs::File;
    use std::io::Write;

    use anyhow::Result;

    use super::*;

    #[test]
    fn test_walk_directory() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let file_path1 = dir.path().join("test1.md");

        let mut file1 = File::create(&file_path1)?;
        writeln!(file1, "This is test1 https://google.com more text")?;
        drop(file1);

        let file_path2 = dir.path().join("test2.md");
        let mut file2 = File::create(&file_path2)?;
        writeln!(file2, "This is test2 with no match.")?;
        drop(file2);

        // Empty file:
        let file_path3 = dir.path().join("test3.txt");
        File::create(&file_path3)?;

        walk_directory(dir.path(), replace::replace_in_file)?;

        let contents1 = std::fs::read_to_string(&file_path1)?;
        // TODO
        assert_eq!(contents1, "This is test1 https://google.com more text\n");

        let contents2 = std::fs::read_to_string(&file_path2)?;
        assert_eq!(contents2, "This is test2 with no match.\n");

        assert!(file_path3.exists());
        let contents3 = std::fs::read_to_string(&file_path3)?;
        assert_eq!(contents3, "");

        dir.close()?;
        Ok(())
    }
}
