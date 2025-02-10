use std::ffi::OsStr;
use std::ffi::OsString;
use std::path::Path;
use std::path::PathBuf;

// Add `insert` after the file base name and the file extension (if there is
// one); otherwise set it as the extension For example, `file.md` becomes
// `file.new.md`
pub fn extend_extension<P: AsRef<Path>>(filepath: &P, insert: &str) -> PathBuf {
    let filepath = filepath.as_ref();

    let mut new: OsString = OsString::new();

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

#[test]
fn test() {
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
    assert_eq!(extend_extension(&Path::new(""), "new"), Path::new(""));
}
// [P1 file_prefix is still unstable.](https://github.com/john-cd/rust_howto/issues/902)
