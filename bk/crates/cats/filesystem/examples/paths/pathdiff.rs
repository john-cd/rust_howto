// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compute a relative path between two paths.
//!
//! A relative path describes how to get from one directory to another, for
//! example `../docs/readme.md`. It is useful in generated links and
//! configuration that must keep working when a project folder is moved.

use std::path::Path;
use std::path::PathBuf;

use pathdiff::diff_paths;

/// Returns the path that leads from `base` to `path`, or `None` if no
///
/// relative path exists (when `path` is relative and `base` is absolute).
///
/// # Arguments
///
/// * `path` - Filesystem location the operation reads, updates, or uses as its scope.
///
/// # Returns
///
/// `Some` when the requested value is available; otherwise returns `None`.
fn relative_to(path: &Path, base: &Path) -> Option<PathBuf> {
    // `diff_paths` compares the components lexically. It does not touch the
    // filesystem, so neither path needs to exist and symbolic links are not
    // resolved.
    diff_paths(path, base)
}

/// Prints a few relative paths.
pub fn example_body() {
    let site = PathBuf::from(if cfg!(windows) {
        r"C:\srv\site"
    } else {
        "/srv/site"
    });
    let target = site.join("assets").join("logo.png");
    let page_dir = site.join("blog").join("2026");

    // Walks up out of `base`, then down to `path`.
    let link = relative_to(&target, &page_dir);
    println!("{link:?}");
    let expected = PathBuf::from("..")
        .join("..")
        .join("assets")
        .join("logo.png");
    assert_eq!(link, Some(expected));

    // A path below `base` needs no `..`.
    let below = relative_to(&page_dir.join("post.md"), &page_dir);
    assert_eq!(below, Some(PathBuf::from("post.md")));

    // A relative path cannot be expressed relative to an absolute base.
    let relative_target = PathBuf::from("assets").join("logo.png");
    assert_eq!(relative_to(&relative_target, &page_dir), None);
    // An absolute path is returned unchanged when the base is relative.
    assert_eq!(relative_to(&target, Path::new("blog")), Some(target));
}
// ANCHOR_END: example

/// Runs the example function.
pub fn run() {
    example_body();
}

/// Returns an empty path when both paths identify the same location.
#[cfg(test)]
mod tests {
    use super::*;

    /// Computes relative paths for upward and downward traversal.
    #[test]
    fn walks_up_and_down() {
        let root = PathBuf::from(if cfg!(windows) { r"C:\a" } else { "/a" });
        let target = root.join("b").join("c");
        let base = root.join("d").join("e");
        let expected = PathBuf::from("..").join("..").join("b").join("c");
        assert_eq!(relative_to(&target, &base), Some(expected));
    }

    /// Returns an empty path when both paths identify the same location.
    #[test]
    fn identical_paths_give_an_empty_path() {
        let path = PathBuf::from(if cfg!(windows) { r"C:\a\b" } else { "/a/b" });
        assert_eq!(relative_to(&path, &path), Some(PathBuf::new()));
    }

    /// Runs succeeds.
    #[test]
    fn run_succeeds() {
        run();
    }
}
