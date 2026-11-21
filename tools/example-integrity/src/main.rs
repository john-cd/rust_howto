//! Command-line entry point for chapter, include, and example integrity reports.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeSet;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context as _;
use anyhow::Result;
use clap::Parser;
use clap::Subcommand;
use example_integrity::build_report;
use example_integrity::check_example_index;
use example_integrity::check_example_refdefs;
use example_integrity::check_local_reference_includes;
use example_integrity::check_subchapter_table_includes;
use example_integrity::generate_example_index;
use example_integrity::generate_example_refdefs;
use example_integrity::survey_example_tests;
use walkdir::WalkDir;

/// Read-only chapter, include, and Cargo target inventory.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Repository root to scan. Defaults to the repository containing this tool.
    #[arg(long, global = true)]
    repo_root: Option<PathBuf>,

    /// Return a failing exit status when the selected report contains problems.
    #[arg(long, global = true)]
    fail: bool,

    /// Render the selected report as CSV when supported.
    #[arg(long, global = true)]
    csv: bool,

    #[command(subcommand)]
    operation: Option<Operation>,
}

/// Additional report views that preserve the behavior of retired scripts.
#[derive(Debug, Subcommand)]
enum Operation {
    /// Survey Rust example test conventions (informational unless `--fail` is set).
    Tests {
        /// Print only per-crate totals.
        #[arg(long)]
        summary: bool,

        /// Optional repository-relative scan roots; defaults to the legacy roots.
        roots: Vec<PathBuf>,
    },

    /// List Rust files in a book's `crates/` tree that no book source includes.
    UnusedExamples {
        /// Book directory relative to the repository root.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,

        /// Print only the number of unused files.
        #[arg(long)]
        summary: bool,
    },

    /// Generate or check the example index and derived example references.
    Index {
        #[command(subcommand)]
        command: IndexCommand,
    },

    /// Audit local reference and subchapter table includes.
    Includes {
        #[command(subcommand)]
        command: IncludeCommand,
    },
}

/// A generation or validation operation for book indexes.
#[derive(Debug, Subcommand)]
enum IndexCommand {
    /// Generate the chapter index from the published Summary.
    Generate {
        /// Book directory containing `src/SUMMARY.md`.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,
    },

    /// Check whether the chapter index matches the published Summary.
    Check {
        /// Book directory containing `src/SUMMARY.md`.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,
    },

    /// Generate or check example reference definitions.
    Refdefs {
        #[command(subcommand)]
        command: RefdefsCommand,
    },
}

/// A generation or validation operation for example reference definitions.
#[derive(Debug, Subcommand)]
enum RefdefsCommand {
    /// Generate derived references while preserving hand-written entries.
    Generate {
        /// Book directory containing `src/indices/refs.incl.md`.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,
    },

    /// Check derived references without changing files.
    Check {
        /// Book directory containing `src/indices/refs.incl.md`.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,
    },
}

/// A read-only audit of one kind of Markdown include.
#[derive(Debug, Subcommand)]
enum IncludeCommand {
    /// Find source files without `{{#include refs.incl.md}}`.
    LocalRefs {
        /// Book directory containing the source `src/` tree.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,
    },

    /// Find nested chapters without their corresponding local recipe table.
    SubchapterTables {
        /// Book directory containing the source `src/` tree.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,
    },
}

/// Returns the repository root containing this package.
fn default_repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf)
}

/// Resolves a user-selected path relative to the repository root when needed.
fn resolve_path(repo_root: &Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        repo_root.join(path)
    }
}

/// Scans the repository and writes the selected report to stdout.
///
/// # Errors
///
/// Returns an error if repository scanning or output fails.
fn run() -> Result<ExitCode> {
    run_with(Args::parse())
}

/// Runs the selected report with parsed arguments.
///
/// # Arguments
///
/// * `args` - Parsed options selecting the report, repository root, and failure behavior.
///
/// # Returns
///
/// A failure exit code when `--fail` is set and the selected report has problems.
///
/// # Errors
///
/// Returns an error if scanning or writing the report fails.
fn run_with(args: Args) -> Result<ExitCode> {
    let repo_root = args.repo_root.unwrap_or_else(default_repository_root);
    let repo_root = repo_root
        .canonicalize()
        .with_context(|| format!("could not resolve repository root {}", repo_root.display()))?;
    match args.operation {
        None => run_inventory(&repo_root, args.csv, args.fail),
        Some(Operation::Tests { summary, roots }) => {
            let survey = survey_example_tests(&repo_root, &roots)?;
            print!("{}", survey.render(summary, args.csv));
            Ok(if args.fail && survey.has_problems() {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        }
        Some(Operation::UnusedExamples { book_root, summary }) => {
            let book_root = if book_root.is_absolute() {
                book_root
            } else {
                repo_root.join(book_root)
            };
            let unused = unused_examples(&repo_root, &book_root)?;
            if args.csv {
                for path in &unused {
                    println!("unused-example,{path}");
                }
            } else if summary {
                println!("{} unused examples", unused.len());
            } else {
                for path in &unused {
                    println!("{path}");
                }
                println!("DONE");
            }
            Ok(if args.fail && !unused.is_empty() {
                ExitCode::FAILURE
            } else {
                ExitCode::SUCCESS
            })
        }
        Some(Operation::Index { command }) => run_index(&repo_root, command),
        Some(Operation::Includes { command }) => Ok(run_includes(&repo_root, command)),
    }
}

/// Runs one include audit, using distinct exit codes for findings and invalid input.
fn run_includes(repo_root: &Path, command: IncludeCommand) -> ExitCode {
    let findings = match command {
        IncludeCommand::LocalRefs { book_root } => {
            check_local_reference_includes(&resolve_path(repo_root, book_root))
        }
        IncludeCommand::SubchapterTables { book_root } => {
            check_subchapter_table_includes(&resolve_path(repo_root, book_root))
        }
    };
    match findings {
        Ok(findings) => {
            print_findings(&findings);
            exit_for_findings(&findings)
        }
        Err(error) => {
            eprintln!("include audit could not run: {error:#}");
            ExitCode::from(2)
        }
    }
}

/// Generates or checks the example index and its derived reference definitions.
///
/// # Errors
///
/// Returns an error if source files cannot be read or generated files cannot be replaced.
fn run_index(repo_root: &Path, command: IndexCommand) -> Result<ExitCode> {
    match command {
        IndexCommand::Generate { book_root } => {
            let book_root = resolve_path(repo_root, book_root);
            let changed = generate_example_index(&book_root)?;
            let target = book_root.join("src/indices/examples_index.md");
            if changed {
                println!("Updated {}", target.display());
            } else {
                println!("Already up to date: {}", target.display());
            }
            Ok(ExitCode::SUCCESS)
        }
        IndexCommand::Check { book_root } => {
            let book_root = resolve_path(repo_root, book_root);
            let findings = check_example_index(&book_root)?;
            print_findings(&findings);
            Ok(exit_for_findings(&findings))
        }
        IndexCommand::Refdefs { command } => match command {
            RefdefsCommand::Generate { book_root } => {
                let book_root = resolve_path(repo_root, book_root);
                let changed = generate_example_refdefs(&book_root)?;
                let target = book_root.join("src/indices/refs.incl.md");
                if changed {
                    println!("Updated {}", target.display());
                } else {
                    println!("Already up to date: {}", target.display());
                }
                Ok(ExitCode::SUCCESS)
            }
            RefdefsCommand::Check { book_root } => {
                let book_root = resolve_path(repo_root, book_root);
                let findings = check_example_refdefs(&book_root)?;
                print_findings(&findings);
                Ok(exit_for_findings(&findings))
            }
        },
    }
}

/// Writes each generated-index finding to stderr.
fn print_findings(findings: &[String]) {
    for finding in findings {
        eprintln!("{finding}");
    }
    if findings.is_empty() {
        println!("Checks passed.");
    }
}

/// Returns a failing status when an audit produced findings.
fn exit_for_findings(findings: &[impl Sized]) -> ExitCode {
    if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Writes the existing stable JSON inventory or a compact diagnostic CSV.
fn run_inventory(repo_root: &Path, csv: bool, fail: bool) -> Result<ExitCode> {
    let report = build_report(repo_root)
        .with_context(|| format!("could not scan {}", repo_root.display()))?;
    if csv {
        println!("severity,code,path,line,message");
        for diagnostic in &report.diagnostics {
            println!(
                "{},{},{},{},{}",
                csv_field(&diagnostic.severity),
                csv_field(&diagnostic.code),
                csv_field(&diagnostic.path),
                diagnostic
                    .line
                    .map_or_else(String::new, |line| line.to_string()),
                csv_field(&diagnostic.message)
            );
        }
    } else {
        serde_json::to_writer_pretty(std::io::stdout().lock(), &report)
            .context("could not write the JSON report")?;
        println!();
    }

    Ok(if fail && report.has_errors() {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    })
}

/// Lists the legacy shell command's `.rs` files that are not natively included by either book.
fn unused_examples(repo_root: &Path, book_root: &Path) -> Result<Vec<String>> {
    let book_root = book_root
        .canonicalize()
        .with_context(|| format!("could not resolve book root {}", book_root.display()))?;
    let relative_book = book_root.strip_prefix(repo_root).with_context(|| {
        format!(
            "book root {} is outside the repository",
            book_root.display()
        )
    })?;
    let root_name = path_to_slashes(relative_book);
    let crate_root = book_root.join("crates");
    if !crate_root.is_dir() {
        return Ok(Vec::new());
    }

    let report = build_report(repo_root)?;
    let included = report
        .includes
        .iter()
        .filter(|include| include.kind == "include")
        .filter(|include| {
            include.source_path.starts_with("bk/src/")
                || include.source_path.starts_with("later/src/")
        })
        .filter_map(|include| include.target_path.as_deref())
        .collect::<BTreeSet<_>>();

    let excluded_names = [
        "lib.rs", "main.rs", "mod.rs", "build.rs", "cat.rs", "lang.rs", "std.rs", "topic.rs",
    ];
    let mut unused = Vec::new();
    for entry in WalkDir::new(&crate_root).follow_links(false) {
        let entry = entry.with_context(|| format!("could not walk {}", crate_root.display()))?;
        if entry.file_type().is_dir()
            || entry.file_type().is_symlink()
            || entry.path().extension() != Some(OsStr::new("rs"))
        {
            continue;
        }
        if entry
            .file_name()
            .to_str()
            .is_some_and(|name| excluded_names.contains(&name))
        {
            continue;
        }
        let relative = entry
            .path()
            .strip_prefix(repo_root)
            .with_context(|| format!("{} is outside the repository", entry.path().display()))?;
        let relative = path_to_slashes(relative);
        if !included.contains(relative.as_str()) {
            let book_relative = relative
                .strip_prefix(&format!("{root_name}/"))
                .unwrap_or(&relative);
            unused.push(book_relative.to_owned());
        }
    }
    unused.sort();
    Ok(unused)
}

/// Escapes a field for stable, standards-compliant CSV output.
fn csv_field(value: &str) -> String {
    if value.contains([',', '"', '\n', '\r']) {
        format!("\"{}\"", value.replace('"', "\"\""))
    } else {
        value.to_owned()
    }
}

/// Converts a platform path to the stable slash-separated report representation.
fn path_to_slashes(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

/// Starts the command-line application and reports top-level errors.
///
/// # Returns
///
/// A process status reflecting the outcome of the command.
fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

/// Scans repositories using the legacy JSON command-line shape.
#[cfg(test)]
mod tests {
    use super::*;

    /// The old invocation still accepts a repository root and strict flag.
    #[test]
    fn legacy_inventory_options_parse_without_a_subcommand() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let root = dir.path().to_str().expect("the temporary path is UTF-8");
        let args = Args::try_parse_from(["example-integrity", "--repo-root", root, "--fail"])?;
        assert!(args.fail);
        assert!(args.operation.is_none());
        Ok(())
    }

    /// Global options parse after a subcommand, as used by scripts and Just recipes.
    #[test]
    fn subcommand_accepts_global_options_after_its_name() -> Result<()> {
        let args = Args::try_parse_from([
            "example-integrity",
            "tests",
            "--summary",
            "--repo-root",
            ".",
            "--fail",
        ])?;
        assert!(args.fail);
        assert!(matches!(
            args.operation,
            Some(Operation::Tests { summary: true, .. })
        ));
        Ok(())
    }

    /// CSV fields quote commas, quotes, and newlines.
    #[test]
    fn csv_fields_are_escaped() {
        assert_eq!(csv_field("plain"), "plain");
        assert_eq!(csv_field("a,\"b\""), "\"a,\"\"b\"\"\"");
    }
}
