//! Command-line entry point for rendered book and visible-content checks.
//!
//! TODO Reviewed ??-??-2026.

use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context as _;
use anyhow::Result;
use book_audit::check_playground;
use book_audit::check_preview_links;
use book_audit::check_rendered_links;
use book_audit::check_search_index;
use book_audit::check_visible_todos;
use book_audit::check_word_index;
use clap::Parser;
use clap::Subcommand;

/// Rendered book and visible Markdown integrity checks.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Repository root. Defaults to the repository containing this tool.
    #[arg(long, global = true)]
    repo_root: Option<PathBuf>,

    #[command(subcommand)]
    command: Command,
}

/// Checks for generated book artifacts and published source.
#[derive(Debug, Subcommand)]
enum Command {
    /// Check required rendered pages and authored chapter links.
    RenderedLinks {
        /// Directory containing the generated mdBook HTML.
        html_dir: PathBuf,

        /// Discover every rendered page and validate linked fragments.
        #[arg(long)]
        all_chapters: bool,
    },

    /// Check fragments linked from the rendered word index.
    WordIndex {
        /// Directory containing the generated mdBook HTML.
        html_dir: PathBuf,
    },

    /// Check runnable Playground blocks against the pinned crate list.
    Playground {
        /// Directory containing the rendered book HTML.
        html_dir: PathBuf,

        /// Book source root containing `crates/Cargo.lock` and the crate allowlist.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,
    },

    /// Check chapter links in a staged preview, optionally including deferred categories.
    PreviewLinks {
        /// Directory containing the staged rendered HTML.
        html_dir: PathBuf,

        /// Summary path; defaults to `later/src/SUMMARY.md` under the repository root.
        #[arg(long)]
        summary: Option<PathBuf>,

        /// Include the normally deferred aerospace and cryptography chapters.
        #[arg(long)]
        include_deferred: bool,
    },

    /// Validate the generated mdBook search index and its publication scope.
    SearchIndex {
        /// Book source root whose generated output is under `book/html`.
        #[arg(long, default_value = "bk")]
        book_root: PathBuf,
    },

    /// Reject TODO placeholders visible in the license or published Markdown.
    VisibleTodos,
}

/// Returns the repository root containing this package.
fn default_repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .map_or_else(|| PathBuf::from("."), std::path::Path::to_path_buf)
}

/// Resolves relative CLI paths against the selected repository root.
fn resolve_path(repo_root: &std::path::Path, path: PathBuf) -> PathBuf {
    if path.is_absolute() {
        path
    } else {
        repo_root.join(path)
    }
}

/// Runs the requested book audit and reports all actionable findings.
///
/// # Errors
///
/// Returns an error if the repository root or a requested source artifact cannot be read.
fn run() -> Result<ExitCode> {
    let args = Args::parse();
    let repo_root = args.repo_root.unwrap_or_else(default_repository_root);
    let repo_root = repo_root
        .canonicalize()
        .with_context(|| format!("could not resolve repository root {}", repo_root.display()))?;
    match args.command {
        Command::RenderedLinks {
            html_dir,
            all_chapters,
        } => {
            let failures = check_rendered_links(&resolve_path(&repo_root, html_dir), all_chapters)?;
            print_failures(&failures);
            Ok(exit_for_findings(&failures))
        }
        Command::WordIndex { html_dir } => {
            let failures = check_word_index(&resolve_path(&repo_root, html_dir))?;
            print_failures(&failures);
            Ok(exit_for_findings(&failures))
        }
        Command::Playground {
            html_dir,
            book_root,
        } => {
            let failures = check_playground(
                &resolve_path(&repo_root, html_dir),
                &resolve_path(&repo_root, book_root),
            )?;
            print_failures(&failures);
            Ok(exit_for_findings(&failures))
        }
        Command::PreviewLinks {
            html_dir,
            summary,
            include_deferred,
        } => {
            let summary = summary.unwrap_or_else(|| repo_root.join("later/src/SUMMARY.md"));
            let (failures, checked) = check_preview_links(
                &resolve_path(&repo_root, html_dir),
                &resolve_path(&repo_root, summary),
                include_deferred,
            )?;
            print_failures(&failures);
            println!(
                "Checked {checked} chapters, found {} link findings.",
                failures.len()
            );
            Ok(exit_for_findings(&failures))
        }
        Command::SearchIndex { book_root } => {
            let report = check_search_index(&resolve_path(&repo_root, book_root))?;
            for finding in &report.findings {
                eprintln!("{finding}");
            }
            if report.findings.is_empty() {
                println!(
                    "Checked {} mdBook search index file(s).",
                    report.index_count
                );
            }
            Ok(exit_for_findings(&report.findings))
        }
        Command::VisibleTodos => {
            let findings = check_visible_todos(&repo_root)?;
            for finding in &findings {
                eprintln!(
                    "{}:{}: visible TODO: {}",
                    finding.path, finding.line, finding.text
                );
            }
            Ok(exit_for_findings(&findings))
        }
    }
}

/// Writes rendered-check findings to stdout, matching the previous link checker.
fn print_failures(failures: &[String]) {
    for failure in failures {
        println!("{failure}");
    }
}

/// Returns a process status based on whether findings were reported.
fn exit_for_findings(findings: &[impl Sized]) -> ExitCode {
    if findings.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::FAILURE
    }
}

/// Starts the command-line application and reports top-level errors.
fn main() -> ExitCode {
    match run() {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}
