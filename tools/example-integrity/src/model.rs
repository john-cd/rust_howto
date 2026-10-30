//! Stable, path-aware report records for the integrity inventory.
//!
//! TODO Reviewed ??-??-2026.

use serde::Serialize;

/// The complete, deterministic inventory and its integrity diagnostics.
#[derive(Debug, Default, Serialize)]
pub struct Report {
    /// Markdown pages with their edition and navigation disposition.
    pub chapters: Vec<ChapterRecord>,
    /// Markdown fragments reached through native include directives.
    pub fragments: Vec<FragmentRecord>,
    /// Native and custom include references found in chapter content.
    pub includes: Vec<IncludeRecord>,
    /// Cargo targets declared by the book and future-book workspaces.
    pub cargo_targets: Vec<CargoTargetRecord>,
    /// Rust modules reachable from Cargo targets, including their conditions.
    pub modules: Vec<ModuleRecord>,
    /// Standalone Rust code fences that are not source-file includes.
    pub standalone_snippets: Vec<SnippetRecord>,
    /// Anchored Rust files and their direct or supporting role.
    pub example_sources: Vec<ExampleSourceRecord>,
    /// Explainable dispositions loaded from the exception file.
    pub exceptions: Vec<ExceptionRecord>,
    /// Missing links, missing anchors, or other integrity failures.
    pub diagnostics: Vec<Diagnostic>,
}

impl Report {
    /// Returns whether the report contains a diagnostic with error severity.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.diagnostics
            .iter()
            .any(|diagnostic| diagnostic.severity == "error")
    }
}

/// A Markdown page and its publication, visibility, and example state.
#[derive(Debug, Serialize)]
pub struct ChapterRecord {
    /// Canonical repository-relative page path.
    pub path: String,
    /// Content area: published, draft, or future.
    pub edition: String,
    /// Whether a Summary file links to the page.
    pub listed: bool,
    /// Whether the page is hidden by the book configuration.
    pub hidden: bool,
    /// Whether visible content includes Rust source compiled by a Cargo target.
    pub has_example: bool,
    /// Whether visible content includes a source file compiled by a test target.
    pub has_tested_example: bool,
    /// Number of standalone Rust fences on the page.
    pub standalone_rust_fences: usize,
    /// A human-readable reason when the page has an accepted exception.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
}

/// A Markdown fragment and the pages that include it.
#[derive(Debug, Serialize)]
pub struct FragmentRecord {
    /// Canonical repository-relative fragment path.
    pub path: String,
    /// Canonical repository-relative source pages or fragments.
    pub included_by: Vec<String>,
}

/// One native or custom source reference found in Markdown.
#[derive(Debug, Serialize)]
pub struct IncludeRecord {
    /// Canonical repository-relative Markdown file containing the directive.
    pub source_path: String,
    /// Root chapter whose rendered content contains the directive.
    pub chapter_path: String,
    /// One-based source line number.
    pub line: usize,
    /// Directive kind: native include, rustdoc include, or custom example.
    pub kind: String,
    /// Canonical repository-relative file path, when the directive names one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_path: Option<String>,
    /// Named anchor or line selection from the directive.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub selection: Option<String>,
    /// Custom example name, when the directive uses a symbolic name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_name: Option<String>,
    /// Whether the include is visible in the rendered book.
    pub visible: bool,
    /// Whether the referenced source exists and can be resolved.
    pub resolved: bool,
    /// Cargo targets that compile the referenced source.
    pub cargo_targets: Vec<String>,
    /// Conditional features and target conditions for the source.
    pub conditions: Vec<String>,
}

/// A Cargo target and its source identity.
#[derive(Clone, Debug, Serialize)]
pub struct CargoTargetRecord {
    /// Stable identity based on the package manifest and target name.
    pub identity: String,
    /// Cargo package name.
    pub package: String,
    /// Cargo target name.
    pub name: String,
    /// Cargo target kinds such as example, bin, or lib.
    pub kinds: Vec<String>,
    /// Canonical repository-relative root source path.
    pub source_path: String,
    /// Features required to build the target.
    pub required_features: Vec<String>,
    /// Whether `cargo test --all-targets` exercises this target.
    pub test: bool,
}

/// One reachable Rust module and the conditions under which it is included.
#[derive(Debug, Serialize)]
pub struct ModuleRecord {
    /// Cargo target that reaches the module.
    pub target_identity: String,
    /// Rust module path, beginning with `crate`.
    pub module_path: String,
    /// Canonical repository-relative source file.
    pub source_path: String,
    /// Required Cargo features and `cfg` expressions on the path.
    pub conditions: Vec<String>,
}

/// A Rust code fence that is not expanded from a source file.
#[derive(Debug, Serialize)]
pub struct SnippetRecord {
    /// Canonical repository-relative Markdown page or fragment.
    pub path: String,
    /// One-based line where the code fence opens.
    pub line: usize,
    /// Language label from the code fence.
    pub language: String,
    /// Root chapter that contains this snippet.
    pub chapter_path: String,
    /// Whether the code appears in rendered content.
    pub visible: bool,
}

/// A Rust file anchored for inclusion and its testing or support role.
#[derive(Debug, Serialize)]
pub struct ExampleSourceRecord {
    /// Canonical repository-relative Rust source path.
    pub path: String,
    /// Whether the source defines an `example` anchor.
    pub anchored: bool,
    /// Role: direct include, helper module, reasoned exclusion, or orphan.
    pub role: String,
    /// Cargo targets that compile the source.
    pub cargo_targets: Vec<String>,
    /// Feature and platform conditions for those targets or modules.
    pub conditions: Vec<String>,
    /// Reason for a reasoned exclusion.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disposition: Option<String>,
}

/// One path-specific reason that a chapter or example may be excluded.
#[derive(Clone, Debug, Serialize)]
pub struct ExceptionRecord {
    /// Exact repository-relative path or directory prefix.
    pub path: String,
    /// Reason supplied next to the exception entry.
    pub reason: String,
}

/// A stable diagnostic describing an integrity gap.
#[derive(Debug, Serialize)]
pub struct Diagnostic {
    /// Severity: error or warning.
    pub severity: String,
    /// Machine-readable diagnostic code.
    pub code: String,
    /// Canonical repository-relative path.
    pub path: String,
    /// One-based line number when the diagnostic has a source line.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line: Option<usize>,
    /// Human-readable description of the problem.
    pub message: String,
}
