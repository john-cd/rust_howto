use std::path::Path;

use anyhow::Result;
use walkdir::WalkDir;

use super::replace::replace_in_file;

/// Walks through a directory and processes all markdown files.
///
/// # Arguments
///
/// * `dir` - The directory to walk through.
pub fn walk_directory(dir: &Path) -> Result<()> {
    for entry in WalkDir::new(dir).into_iter().filter_map(|e| e.ok()) {
        let f_name = entry.file_name().to_string_lossy();
        if f_name.ends_with(".md") {
            let path = entry.path();
            replace_in_file(path)?;
        }
    }
    Ok(())
}

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
        let file_path2 = dir.path().join("test2.md");
        let file_path3 = dir.path().join("test3.txt");

        let mut file1 = File::create(&file_path1)?;
        writeln!(file1, "This is test1 with {{{{!crate clap}}}}.")?;
        drop(file1);

        let mut file2 = File::create(&file_path2)?;
        writeln!(file2, "This is test2 with no match.")?;
        drop(file2);

        File::create(&file_path3)?;

        walk_directory(dir.path())?;

        let contents1 = std::fs::read_to_string(&file_path1)?;
        assert_eq!(
            contents1,
            "This is test1 with [![clap][c~clap~docs~badge]][c~clap~docs]{{hi:clap}}.\n"
        );

        let contents2 = std::fs::read_to_string(&file_path2)?;
        assert_eq!(contents2, "This is test2 with no match.\n");

        assert!(file_path3.exists());
        dir.close()?;
        Ok(())
    }
}
