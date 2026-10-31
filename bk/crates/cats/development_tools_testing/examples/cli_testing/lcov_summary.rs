// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Compute line coverage per file from an LCOV report and enforce a minimum.
//!
//! Code coverage reports which lines tests execute. LCOV is a plain-text report
//! format produced by tools such as `cargo llvm-cov`, and parsing it lets a
//! script summarize or enforce a coverage threshold.

use std::io::BufRead;

use lcov::Reader;
use lcov::Record;

/// Line coverage of one source file.
#[derive(Debug, PartialEq)]
struct FileCoverage {
    /// Path of the source file.
    path: String,
    /// Number of instrumented lines.
    found: u32,
    /// Number of lines executed at least once.
    hit: u32,
}

impl FileCoverage {
    /// Returns the percentage of instrumented lines that were executed.
    fn percent(&self) -> f64 {
        if self.found == 0 {
            100.0
        } else {
            f64::from(self.hit) * 100.0 / f64::from(self.found)
        }
    }
}

/// Streams the records of an LCOV report and collects per-file line coverage.
///
/// # Errors
///
/// Returns an error if the report contains malformed LCOV records.
fn summarize(input: impl BufRead) -> anyhow::Result<Vec<FileCoverage>> {
    let mut files = Vec::new();
    let mut current: Option<FileCoverage> = None;
    for record in Reader::new(input) {
        match record? {
            Record::SourceFile { path } => {
                current = Some(FileCoverage {
                    path: path.display().to_string(),
                    found: 0,
                    hit: 0,
                });
            }
            Record::LinesFound { found } => {
                if let Some(file) = current.as_mut() {
                    file.found = found;
                }
            }
            Record::LinesHit { hit } => {
                if let Some(file) = current.as_mut() {
                    file.hit = hit;
                }
            }
            Record::EndOfRecord => files.extend(current.take()),
            _ => {}
        }
    }
    Ok(files)
}

/// Fails if any file is covered below `threshold` percent.
///
/// # Errors
///
/// Returns an error when a file is below the requested coverage threshold.
fn check_threshold(
    files: &[FileCoverage],
    threshold: f64,
) -> anyhow::Result<()> {
    for file in files {
        anyhow::ensure!(
            file.percent() >= threshold,
            "{} has {:.1}% line coverage, below {threshold}%",
            file.path,
            file.percent()
        );
    }
    Ok(())
}

/// A report, as written by `cargo llvm-cov --lcov`.
const REPORT: &str = "\
TN:
SF:src/lib.rs
DA:1,1
DA:2,1
DA:3,0
DA:4,1
LF:4
LH:3
end_of_record
SF:src/main.rs
DA:1,1
DA:2,1
LF:2
LH:2
end_of_record
";

/// Summarizes the report, then applies a coverage gate.
///
/// # Errors
///
/// Returns an error if the report is invalid or below the coverage threshold.
pub fn run() -> anyhow::Result<()> {
    let files = summarize(REPORT.as_bytes())?;
    for file in &files {
        println!("{}: {:.1}%", file.path, file.percent());
    }
    check_threshold(&files, 70.0)
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that the report yields the expected per-file percentages.
    ///
    /// # Errors
    ///
    /// Returns an error if the inline report cannot be parsed.
    #[test]
    fn computes_coverage_per_file() -> anyhow::Result<()> {
        let files = summarize(REPORT.as_bytes())?;
        assert_eq!(files.len(), 2);
        assert_eq!(files[0].path, "src/lib.rs");
        assert!((files[0].percent() - 75.0).abs() < f64::EPSILON);
        assert!((files[1].percent() - 100.0).abs() < f64::EPSILON);
        Ok(())
    }

    /// Verifies that the threshold gate rejects under-covered files.
    ///
    /// # Errors
    ///
    /// Returns an error if the inline report cannot be parsed.
    #[test]
    fn gate_fails_below_the_threshold() -> anyhow::Result<()> {
        let files = summarize(REPORT.as_bytes())?;
        assert!(check_threshold(&files, 90.0).is_err());
        Ok(())
    }

    /// Verifies that malformed LCOV records are rejected.
    #[test]
    fn malformed_input_is_an_error() {
        assert!(summarize(&b"LF:x\n"[..]).is_err());
    }

    /// Verifies that the summary example completes successfully.
    ///
    /// # Errors
    ///
    /// Returns an error if the inline report cannot be summarized.
    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()
    }
}
