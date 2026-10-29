// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Demonstrates how to ignore case when matching file names.

use std::path::PathBuf;

use anyhow::Result;
use glob::MatchOptions;
use glob::glob_with;

/// Prints the paths that match a glob pattern, ignoring case.
fn main() -> Result<()> {
    for path in find_ignoring_case("/media/img_[0-9]*.png")? {
        println!("{}", path.display());
    }

    Ok(())
}

/// Returns the paths that match `pattern`, ignoring case.
fn find_ignoring_case(pattern: &str) -> Result<Vec<PathBuf>> {
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    let mut paths = Vec::new();
    for entry in glob_with(pattern, options)? {
        paths.push(entry?);
    }
    Ok(paths)
}

// ANCHOR_END: example

pub fn run() -> Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Runs the ignore case example through its entry point.
    #[test]
    fn test_ignore_case_example_entry_point() -> anyhow::Result<()> {
        main()?;
        Ok(())
    }

    /// Matching ignores case but still honors the digit class.
    #[test]
    fn matches_regardless_of_case() -> Result<()> {
        let dir = tempfile::tempdir()?;
        for name in ["img_1.png", "IMG_2.PNG", "img_x.png", "img_3.jpg"] {
            std::fs::write(dir.path().join(name), "")?;
        }
        let pattern = format!("{}/img_[0-9]*.png", dir.path().display());
        let mut found = find_ignoring_case(&pattern)?;
        found.sort();
        assert_eq!(
            found,
            vec![dir.path().join("IMG_2.PNG"), dir.path().join("img_1.png")]
        );
        Ok(())
    }
}
