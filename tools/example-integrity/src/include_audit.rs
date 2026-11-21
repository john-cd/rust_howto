//! Audits the book's local reference and subchapter table includes.
//!
//! TODO Reviewed ??-??-2026.

use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::ensure;
use walkdir::WalkDir;

/// Finds book source files that omit their local reference include.
///
/// The scan matches the legacy include audit's roots and filename exclusions.
/// Findings use paths relative to `book_root` and include the expected directive.
///
/// # Arguments
///
/// * `book_root` - Book directory containing the `src/` tree.
///
/// # Errors
///
/// Returns an error when the book source root cannot be resolved or scanned.
pub fn check_local_reference_includes(book_root: &Path) -> Result<Vec<String>> {
    let (book_root, source_root) = book_source_root(book_root)?;
    let mut findings = Vec::new();
    for entry in WalkDir::new(&source_root)
        .follow_links(false)
        .sort_by_file_name()
    {
        let entry = entry.with_context(|| format!("could not walk {}", source_root.display()))?;
        let path = entry.path();
        if !path.is_file() || is_local_reference_exempt(entry.file_name()) {
            continue;
        }
        let content =
            fs::read(path).with_context(|| format!("could not read {}", path.display()))?;
        if !contains_bytes(&content, b"{{#include refs.incl.md}}") {
            findings.push(format!(
                "missing local reference include in {}; expected {{{{#include refs.incl.md}}}}",
                relative_path(&book_root, path)?
            ));
        }
    }
    Ok(findings)
}

/// Finds nested chapter files that omit their local recipe table include.
///
/// Existing includes with the optional leading underscore remain accepted, as in
/// the legacy audit. Findings recommend the non-underscored spelling.
///
/// # Arguments
///
/// * `book_root` - Book directory containing the `src/` tree.
///
/// # Errors
///
/// Returns an error when the book source root or a candidate Markdown file cannot be read.
pub fn check_subchapter_table_includes(book_root: &Path) -> Result<Vec<String>> {
    let (book_root, source_root) = book_source_root(book_root)?;
    let mut findings = Vec::new();
    for entry in WalkDir::new(&source_root)
        .follow_links(false)
        .sort_by_file_name()
    {
        let entry = entry.with_context(|| format!("could not walk {}", source_root.display()))?;
        let path = entry.path();
        if !path.is_file() || is_subchapter_table_exempt(&source_root, path)? {
            continue;
        }

        let stem = path
            .file_stem()
            .with_context(|| {
                format!(
                    "could not determine the chapter name for {}",
                    path.display()
                )
            })?
            .to_string_lossy();
        let content = fs::read_to_string(path)
            .with_context(|| format!("could not read Markdown source {}", path.display()))?;
        let include = format!("{{{{#include {stem}.incl.md}}}}");
        let underscored_include = format!("{{{{#include _{stem}.incl.md}}}}");
        if !content.contains(&include) && !content.contains(&underscored_include) {
            findings.push(format!(
                "missing subchapter table include in {}; consider adding --> {include}",
                relative_path(&book_root, path)?
            ));
        }
    }
    Ok(findings)
}

/// Resolves the book root and verifies that its `src/` directory exists.
fn book_source_root(book_root: &Path) -> Result<(PathBuf, PathBuf)> {
    let book_root = book_root
        .canonicalize()
        .with_context(|| format!("could not resolve book root {}", book_root.display()))?;
    let source_root = book_root.join("src");
    ensure!(
        source_root.is_dir(),
        "book source root {} is not a directory",
        source_root.display()
    );
    Ok((book_root, source_root))
}

/// Applies the legacy local-reference scan's filename exclusions.
fn is_local_reference_exempt(file_name: &OsStr) -> bool {
    let file_name = file_name.to_string_lossy();
    file_name.ends_with(".incl.md")
        || file_name.ends_with("refs.md")
        || file_name.ends_with(".bak")
        || file_name == "word_index.md"
}

/// Applies the legacy subchapter scan's Markdown and path exclusions.
fn is_subchapter_table_exempt(source_root: &Path, path: &Path) -> Result<bool> {
    let file_name = path
        .file_name()
        .with_context(|| format!("could not determine the file name for {}", path.display()))?
        .to_string_lossy();
    if !file_name.ends_with(".md")
        || file_name.ends_with("index.md")
        || file_name.ends_with(".incl.md")
        || file_name.ends_with("refs.md")
    {
        return Ok(true);
    }

    let relative = path
        .strip_prefix(source_root)
        .with_context(|| format!("{} is outside {}", path.display(), source_root.display()))?;
    let parent = relative.parent().unwrap_or_else(|| Path::new(""));
    Ok(parent.as_os_str().is_empty() || parent == Path::new("crates"))
}

/// Checks whether a byte slice contains an exact include directive.
fn contains_bytes(content: &[u8], needle: &[u8]) -> bool {
    content.windows(needle.len()).any(|window| window == needle)
}

/// Converts a path under the book root to stable slash-separated text.
fn relative_path(book_root: &Path, path: &Path) -> Result<String> {
    let relative = path
        .strip_prefix(book_root)
        .with_context(|| format!("{} is outside {}", path.display(), book_root.display()))?;
    Ok(relative
        .components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/"))
}

/// Verifies include audit findings, accepted spellings, and legacy path exclusions.
#[cfg(test)]
mod tests {
    use tempfile::tempdir;

    use super::*;

    /// Reports missing local references while accepting the exact directive and excluding support
    /// files.
    #[test]
    fn local_reference_audit_preserves_legacy_exclusions() -> Result<()> {
        let directory = tempdir()?;
        let source = directory.path().join("src/sections");
        fs::create_dir_all(&source)?;
        fs::write(source.join("missing.md"), "# Missing\n")?;
        fs::write(
            source.join("ready.md"),
            "# Ready\n{{#include refs.incl.md}}\n",
        )?;
        fs::write(source.join("table.incl.md"), "table support\n")?;
        fs::write(source.join("chapter.refs.md"), "reference support\n")?;
        fs::write(source.join("backup.md.bak"), "backup\n")?;
        fs::write(source.join("word_index.md"), "generated index\n")?;

        assert_eq!(
            check_local_reference_includes(directory.path())?,
            [
                "missing local reference include in src/sections/missing.md; expected {{#include refs.incl.md}}"
            ]
        );
        Ok(())
    }

    /// Reports nested subchapters and accepts the optional leading underscore in table includes.
    #[test]
    fn subchapter_audit_preserves_legacy_roots_and_exclusions() -> Result<()> {
        let directory = tempdir()?;
        let src = directory.path().join("src");
        fs::create_dir_all(src.join("chapters"))?;
        fs::create_dir_all(src.join("crates"))?;
        fs::write(src.join("chapters/missing.md"), "# Missing\n")?;
        fs::write(
            src.join("chapters/ready.md"),
            "# Ready\n{{#include _ready.incl.md}}\n",
        )?;
        fs::write(src.join("chapters/table.incl.md"), "table support\n")?;
        fs::write(src.join("chapters/refs.md"), "reference support\n")?;
        fs::write(src.join("chapters/generated_index.md"), "generated index\n")?;
        fs::write(src.join("top_level.md"), "# Top level\n")?;
        fs::write(src.join("crates/demo.md"), "# Crate page\n")?;

        assert_eq!(
            check_subchapter_table_includes(directory.path())?,
            [
                "missing subchapter table include in src/chapters/missing.md; consider adding --> {{#include missing.incl.md}}"
            ]
        );
        Ok(())
    }

    /// Rejects a repository path whose book source directory is missing.
    #[test]
    fn include_audits_reject_a_missing_source_root() -> Result<()> {
        let directory = tempdir()?;
        let error = check_local_reference_includes(directory.path())
            .expect_err("a book without src should be invalid");
        assert!(error.to_string().contains("src"));
        Ok(())
    }
}
