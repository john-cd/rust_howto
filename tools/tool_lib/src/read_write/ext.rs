//! Manipulating file extensions.
//!
//! TODO Reviewed ??-??-2026.

use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

/// Add `insert` after the file base name and before the file extension (if there is
/// one); otherwise set it as the extension. For example, `file.md` becomes
/// `file.new.md`
///
/// # Examples
///
/// ```rust
/// use std::path::Path;
/// use tool_lib::extend_extension;
///
/// assert_eq!(extend_extension(&Path::new("file.md"), "new"), Path::new("file.new.md"));
/// assert_eq!(extend_extension(&Path::new("file"), "new"), Path::new("file.new"));
/// ```
pub fn extend_extension<P: AsRef<Path>>(filepath: &P, insert: &str) -> PathBuf {
    let filepath = filepath.as_ref();

    let mut new = OsString::new();

    if let Some(file_stem) = filepath.file_stem() {
        new.push(file_stem);
        if !insert.is_empty() {
            new.push(OsStr::new("."));
            new.push(OsStr::new(insert));
        }
        // The extension is: None, if there is no file name; if there is no
        // embedded `.``; if the file name begins with `.` and has no
        // other `.`s within.
        let ext: Option<&OsStr> = filepath.extension();
        if let Some(e) = ext {
            new.push(OsStr::new("."));
            new.push(e);
        }
        match filepath.parent() {
            Some(p) => p.join(PathBuf::from(new)),
            None => PathBuf::from(new),
        }
    } else {
        // file_stem is None, if there is no file name;
        PathBuf::from(filepath)
    }
}

// `Path::file_prefix` is stable, but it is deliberately not used above: it
// stops at the first `.` (`a.tar.gz` -> `a`), whereas `file_stem` keeps
// everything before the last `.` (`a.tar.gz` -> `a.tar`), which is what
// `extend_extension` needs to preserve inner dots.

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the ext example's main path succeeds.
    #[test]
    fn test_main() {
        assert_eq!(
            extend_extension(&Path::new("/path/to/file.md"), "new"),
            Path::new("/path/to/file.new.md")
        );
        assert_eq!(
            extend_extension(&Path::new("/path/to/file"), "new"),
            Path::new("/path/to/file.new")
        );
        assert_eq!(
            extend_extension(&Path::new("file"), "new"),
            Path::new("file.new")
        );
        assert_eq!(
            extend_extension(&Path::new("file.md"), "new"),
            Path::new("file.new.md")
        );
        assert_eq!(
            extend_extension(&Path::new("file.a.md"), "new"),
            Path::new("file.a.new.md")
        );
        assert_eq!(
            extend_extension(&Path::new("a.tar.gz"), "new"),
            Path::new("a.tar.new.gz")
        );
        assert_eq!(extend_extension(&Path::new(""), "new"), Path::new(""));
    }
}
