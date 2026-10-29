// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compute a relative path between two paths.

use std::path::Path;
use std::path::PathBuf;

use pathdiff::diff_paths;

/// Returns the path that leads from `base` to `path`, or `None` if no
/// relative path exists (when `path` is relative and `base` is absolute).
fn relative_to(path: &Path, base: &Path) -> Option<PathBuf> {
    // `diff_paths` compares the components lexically. It does not touch the
    // filesystem, so neither path needs to exist and symbolic links are not
    // resolved.
    diff_paths(path, base)
}

/// Prints a few relative paths.
pub fn run() {
    let target = Path::new("/srv/site/assets/logo.png");
    let page_dir = Path::new("/srv/site/blog/2026");

    // Walks up out of `base`, then down to `path`.
    let link = relative_to(target, page_dir);
    println!("{link:?}");
    assert_eq!(link, Some(PathBuf::from("../../assets/logo.png")));

    // A path below `base` needs no `..`.
    let below = relative_to(Path::new("/srv/site/blog/2026/post.md"), page_dir);
    assert_eq!(below, Some(PathBuf::from("post.md")));

    // A relative path cannot be expressed relative to an absolute base.
    assert_eq!(relative_to(Path::new("assets/logo.png"), page_dir), None);
    // An absolute path is returned unchanged when the base is relative.
    assert_eq!(
        relative_to(target, Path::new("blog")),
        Some(target.to_path_buf())
    );
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walks_up_and_down() {
        let relative = relative_to(Path::new("/a/b/c"), Path::new("/a/d/e"));
        assert_eq!(relative, Some(PathBuf::from("../../b/c")));
    }

    #[test]
    fn identical_paths_give_an_empty_path() {
        let relative = relative_to(Path::new("/a/b"), Path::new("/a/b"));
        assert_eq!(relative, Some(PathBuf::new()));
    }

    #[test]
    fn run_succeeds() {
        run();
    }
}
