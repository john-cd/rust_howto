// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Find reference-style links that have no definition with `mdbook-utils`.
//!
//! Install the tool with `cargo install mdbook-utils`; see its user guide for
//! the other subcommands (`refdefs`, `markdown`, `sitemap`).

use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::Context;
use anyhow::Result;

/// Runs `mdbook-utils links broken-links` on `markdown_dir` and returns the
/// report that the tool writes.
///
/// # Errors
///
/// Returns an error if `mdbook-utils` is not installed, exits unsuccessfully,
/// or does not write the report.
fn find_broken_links(markdown_dir: &Path) -> Result<String> {
    let report_path = markdown_dir.join("broken.md");
    let status = Command::new("mdbook-utils")
        .args(["links", "broken-links", "--yes", "--markdown-dir"])
        .arg(markdown_dir)
        .arg("--output")
        .arg(&report_path)
        .status()
        .context(
            "run `mdbook-utils` (install it with `cargo install mdbook-utils`)",
        )?;
    anyhow::ensure!(status.success(), "mdbook-utils failed: {status}");
    fs::read_to_string(&report_path).context("read the broken-links report")
}

/// Writes a small book chapter with one undefined link, then reports it.
fn main() -> Result<()> {
    // The tool skips hidden directories, so give the directory a visible name.
    let book = tempfile::Builder::new().prefix("book").tempdir()?;
    fs::write(
        book.path().join("page.md"),
        "# Title\n\nSee [the docs][docs] and [missing][nope].\n\n[docs]: https://example.com\n",
    )?;
    let report = find_broken_links(book.path())?;
    println!("{report}");
    Ok(())
}
// ANCHOR_END: example

/// Runs the `mdbook-utils` broken-links example.
///
/// # Errors
///
/// Returns an error if the tool is missing or fails.
pub fn run() -> Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that only the undefined reference is reported; ignored when the
    /// tool is not installed.
    #[test]
    #[ignore = "needs `mdbook-utils` on the PATH (cargo install mdbook-utils)"]
    fn reports_only_the_undefined_reference() -> Result<()> {
        let book = tempfile::Builder::new().prefix("book").tempdir()?;
        fs::write(
            book.path().join("page.md"),
            "See [a][known] and [b][unknown].\n\n[known]: https://example.com\n",
        )?;
        let report = find_broken_links(book.path())?;
        assert!(report.contains("Reference: unknown"));
        assert!(!report.contains("Reference: known"));
        Ok(())
    }

    #[test]
    #[ignore = "needs `mdbook-utils` on the PATH (cargo install mdbook-utils)"]
    fn test_main_entry_point() -> Result<()> {
        main()
    }

    #[test]
    fn missing_tool_or_directory_is_reported_as_an_error() {
        let dir =
            tempfile::tempdir().expect("temporary directory should be created");
        let nested = dir.path().join("does-not-exist");
        // Either `mdbook-utils` is absent, or it fails on the missing
        // directory.
        assert!(find_broken_links(&nested).is_err());
    }
}
