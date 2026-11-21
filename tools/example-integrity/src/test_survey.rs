//! Audits the repository's example entry-point and test conventions.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeMap;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::sync::LazyLock;

use anyhow::Context as _;
use anyhow::Result;
use regex::Regex;
use walkdir::WalkDir;

/// Default roots scanned by the example test-convention survey.
pub const DEFAULT_TEST_ROOTS: [&str; 4] = ["bk/crates", "later/crates", "xmpl", "tools"];

/// Directories excluded from the recursive example source scan.
const SKIP_PARTS: [&str; 3] = ["target", ".git", "node_modules"];

/// Finds a Rust `main` entry point.
static FN_MAIN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?fn\s+main\s*\(")
        .expect("the main function regex is valid")
});

/// Finds a public Rust `run` entry point used by multi-file examples.
static FN_RUN: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?m)^\s*pub(?:\([^)]*\))?\s+(?:async\s+)?fn\s+run\s*\(")
        .expect("the run function regex is valid")
});

/// Finds an individual or qualified Rust test attribute.
static TEST_ATTR: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"#\[(?:\w+::)*test\b").expect("the test attribute regex is valid")
});

/// Finds a test call to the example entry point or its injected argument helper.
static MAIN_CALL: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"(?:^|[^\w:])(?:super::)?main\s*\(\s*\)").expect("the main call regex is valid")
});

/// Finds ignored tests whose reason identifies an external requirement.
static IGNORED_EXTERNAL_TEST: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r#"(?i)#\[\s*ignore\s*=\s*"(?:requires?|needs?)\b[^"]*"\s*\]"#)
        .expect("the ignored external test regex is valid")
});

/// Finds tests that call the parameterized example entry helper.
static INJECTED_ENTRY: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\b(?:run_with_args|run_with_output_path)\s*\(")
        .expect("the injected entry helper regex is valid")
});

/// Finds assertion helpers and output-only markers.
static ENSURE: LazyLock<Regex> =
    LazyLock::new(|| Regex::new(r"\bensure!\s*\(").expect("the ensure macro regex is valid"));

/// Finds `expect` or `unwrap` checks in the test section.
static EXPECT_OR_UNWRAP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(r"\.(?:expect|unwrap)\s*\(").expect("the expect/unwrap regex is valid")
});

/// One example source that defines `main` and the reasons it does not meet the convention.
#[derive(Debug, Eq, PartialEq)]
pub struct ExampleTestRecord {
    /// Cargo crate directory containing this source file.
    pub crate_root: String,

    /// Repository-relative source path with slash separators.
    pub path: String,

    /// Convention violations in the same order as the original survey.
    pub problems: Vec<String>,
}

/// Results from scanning all configured example roots.
#[derive(Debug, Default)]
pub struct ExampleTestSurvey {
    /// Source records sorted by crate root and repository-relative path.
    pub records: Vec<ExampleTestRecord>,
}

impl ExampleTestSurvey {
    /// Reports whether the survey found a convention violation.
    #[must_use]
    pub fn has_problems(&self) -> bool {
        self.records
            .iter()
            .any(|record| !record.problems.is_empty())
    }

    /// Renders the survey as detail, per-crate counts, or CSV problem rows.
    #[must_use]
    pub fn render(&self, summary: bool, csv: bool) -> String {
        if csv {
            return self.render_csv();
        }
        self.render_text(summary)
    }

    /// Renders one CSV row for each source with one or more problems.
    fn render_csv(&self) -> String {
        let mut output = String::new();
        for record in &self.records {
            if !record.problems.is_empty() {
                output.push_str(&record.path);
                output.push(',');
                output.push_str(&record.problems.join("; "));
                output.push('\n');
            }
        }
        output
    }

    /// Renders deterministic per-crate counts and optional problem details.
    fn render_text(&self, summary: bool) -> String {
        let mut totals = BTreeMap::<&str, (usize, usize)>::new();
        for record in &self.records {
            let counts = totals.entry(&record.crate_root).or_default();
            counts.0 += 1;
            counts.1 += usize::from(record.problems.is_empty());
        }

        let mut output = String::new();
        let mut grand = (0, 0);
        for (crate_root, (files, compliant)) in totals {
            grand.0 += files;
            grand.1 += compliant;
            output.push_str(&format!("{crate_root}: {compliant}/{files} compliant\n"));
            if !summary {
                for record in self
                    .records
                    .iter()
                    .filter(|record| record.crate_root == crate_root && !record.problems.is_empty())
                {
                    output.push_str(&format!(
                        "    {}: {}\n",
                        record.path,
                        record.problems.join("; ")
                    ));
                }
            }
        }
        output.push_str(&format!("TOTAL: {}/{} compliant\n", grand.1, grand.0));
        output
    }
}

/// Scans example roots for files defining `main` and evaluates their test conventions.
///
/// # Arguments
///
/// * `repo_root` - Repository root used to resolve configured roots and output paths.
/// * `roots` - Optional scan roots relative to the repository; an empty list selects defaults.
///
/// # Errors
///
/// Returns an error if a requested root cannot be scanned or a source path is outside the repo.
pub fn survey_example_tests(repo_root: &Path, roots: &[PathBuf]) -> Result<ExampleTestSurvey> {
    let repo_root = repo_root
        .canonicalize()
        .with_context(|| format!("could not resolve repository root {}", repo_root.display()))?;
    let default_roots = DEFAULT_TEST_ROOTS.map(PathBuf::from);
    let roots = if roots.is_empty() {
        &default_roots[..]
    } else {
        roots
    };
    let mut sources = Vec::new();
    for root in roots {
        let scan_root = if root.is_absolute() {
            root.clone()
        } else {
            repo_root.join(root)
        };
        if !scan_root.is_dir() {
            continue;
        }
        for entry in WalkDir::new(&scan_root).follow_links(false) {
            let entry = entry.with_context(|| format!("could not walk {}", scan_root.display()))?;
            if entry.file_type().is_dir() || !entry.path().is_file() {
                continue;
            }
            if entry
                .path()
                .extension()
                .is_none_or(|extension| extension != "rs")
            {
                continue;
            }
            let relative = entry.path().strip_prefix(&repo_root).with_context(|| {
                format!("{} is outside the repository root", entry.path().display())
            })?;
            let parts = relative
                .components()
                .map(|part| part.as_os_str().to_string_lossy().into_owned())
                .collect::<Vec<_>>();
            if parts.iter().any(|part| SKIP_PARTS.contains(&part.as_str()))
                || !parts
                    .iter()
                    .any(|part| matches!(part.as_str(), "examples" | "bin" | "src"))
                || parts.iter().any(|part| part == "tests")
                || entry.file_name() == "build.rs"
            {
                continue;
            }
            sources.push(entry.path().to_path_buf());
        }
    }
    sources.sort();
    sources.dedup();

    let mut survey = ExampleTestSurvey::default();
    for source_path in sources {
        let bytes = std::fs::read(&source_path)
            .with_context(|| format!("could not read example source {}", source_path.display()))?;
        let text = String::from_utf8_lossy(&bytes);
        let Some(problems) = classify_source(
            text.as_ref(),
            source_path
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or_default(),
            source_path.parent().is_some_and(|parent| {
                parent.join("main.rs").is_file()
                    && source_path.file_name() != Some(OsStr::new("main.rs"))
            }),
        ) else {
            continue;
        };
        let relative = source_path
            .strip_prefix(&repo_root)
            .context("example source is outside the repository root")?;
        survey.records.push(ExampleTestRecord {
            crate_root: cargo_crate_root(&repo_root, &source_path),
            path: path_to_slashes(relative),
            problems,
        });
    }
    survey.records.sort_by(|left, right| {
        (&left.crate_root, &left.path).cmp(&(&right.crate_root, &right.path))
    });
    Ok(survey)
}

/// Returns convention violations for a source file, or `None` when it does not define `main`.
fn classify_source(text: &str, file_name: &str, multi_file_member: bool) -> Option<Vec<String>> {
    if !FN_MAIN.is_match(text) {
        return None;
    }
    let has_run = FN_RUN.is_match(text);
    let cfg_test = text.find("#[cfg(test)]");
    let has_test = cfg_test.is_some() || TEST_ATTR.is_match(text);
    let test_text = cfg_test
        .and_then(|start| text.get(start..))
        .unwrap_or_default();
    let ignored_external_test = IGNORED_EXTERNAL_TEST.is_match(test_text);
    let calls_main = MAIN_CALL.is_match(test_text)
        || INJECTED_ENTRY.is_match(test_text)
        || ignored_external_test;
    let lowered = text.to_ascii_lowercase();
    let has_assertion = ["assert", "should_panic", "output-only"]
        .iter()
        .any(|needle| lowered.contains(needle))
        || ENSURE.is_match(text)
        || EXPECT_OR_UNWRAP.is_match(test_text)
        || ignored_external_test;

    let mut problems = Vec::new();
    if multi_file_member && !has_run {
        problems.push("missing pub fn run()".to_owned());
    }
    if !has_test {
        problems.push("no tests".to_owned());
    } else if !calls_main && file_name != "main.rs" {
        problems.push("no test calls main()".to_owned());
    } else if !has_assertion {
        problems.push("tests make no assertions".to_owned());
    }
    Some(problems)
}

/// Returns the nearest Cargo manifest directory relative to the repository root.
fn cargo_crate_root(repo_root: &Path, source_path: &Path) -> String {
    source_path
        .parent()
        .into_iter()
        .flat_map(Path::ancestors)
        .find(|directory| directory.join("Cargo.toml").is_file())
        .and_then(|directory| directory.strip_prefix(repo_root).ok())
        .map_or_else(|| "?".to_owned(), path_to_slashes)
}

/// Converts a platform path to the stable slash-separated report representation.
fn path_to_slashes(path: &Path) -> String {
    path.components()
        .map(|component| component.as_os_str().to_string_lossy())
        .collect::<Vec<_>>()
        .join("/")
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Counts an `ensure!` assertion in `main` when the test calls `main`.
    #[test]
    fn main_level_ensure_counts_as_a_behavior_check() {
        let source = "fn main() { anyhow::ensure!(true, \"condition failed\"); }\n#[cfg(test)]\nmod tests { #[test] fn check() { let _ = super::main(); } }";
        let problems =
            classify_source(source, "example.rs", false).expect("the example defines main");
        assert!(problems.is_empty());
    }

    /// Accepts an ignored test whose reason names an external service requirement.
    #[test]
    fn ignored_external_test_counts_as_a_behavior_check() {
        let source = "fn main() {}\n#[cfg(test)]\nmod tests { #[test] #[ignore = \"needs a reachable service\"] fn service() { helper(); } }";
        let problems =
            classify_source(source, "example.rs", false).expect("the example defines main");
        assert!(problems.is_empty());
    }

    /// Requires a multi-file member to expose `run` and exercise `main` from its test.
    #[test]
    fn multi_file_member_reports_missing_run_and_main_call() {
        let source =
            "fn main() {}\n#[cfg(test)]\nmod tests { #[test] fn check() { assert!(true); } }";
        let problems =
            classify_source(source, "helper.rs", true).expect("the example defines main");
        assert_eq!(problems, ["missing pub fn run()", "no test calls main()"]);
    }

    /// Uses deterministic crate totals and emits only problem records in CSV mode.
    #[test]
    fn survey_formats_summary_and_csv_deterministically() {
        let survey = ExampleTestSurvey {
            records: vec![
                ExampleTestRecord {
                    crate_root: "bk/crates/demo".to_owned(),
                    path: "bk/crates/demo/examples/ok.rs".to_owned(),
                    problems: Vec::new(),
                },
                ExampleTestRecord {
                    crate_root: "bk/crates/demo".to_owned(),
                    path: "bk/crates/demo/examples/missing.rs".to_owned(),
                    problems: vec!["no tests".to_owned()],
                },
            ],
        };
        assert_eq!(
            survey.render(true, false),
            "bk/crates/demo: 1/2 compliant\nTOTAL: 1/2 compliant\n"
        );
        assert_eq!(
            survey.render(false, true),
            "bk/crates/demo/examples/missing.rs,no tests\n"
        );
    }

    /// Scans only example sources under selected roots and reports the crate containing each.
    #[test]
    fn scan_filters_support_files_and_finds_example_tests() -> Result<()> {
        let directory = tempfile::tempdir()?;
        let root = directory.path();
        let example = root.join("bk/crates/demo/examples/main.rs");
        std::fs::create_dir_all(example.parent().context("example has a parent")?)?;
        std::fs::write(
            root.join("bk/crates/demo/Cargo.toml"),
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2024\"\n",
        )?;
        std::fs::write(
            &example,
            "fn main() { assert!(true); }\n#[cfg(test)] mod tests { #[test] fn works() { super::main(); } }",
        )?;
        std::fs::write(
            root.join("bk/crates/demo/examples/build.rs"),
            "fn main() {}",
        )?;
        let roots = [PathBuf::from("bk/crates")];
        let survey = survey_example_tests(root, &roots)?;
        assert_eq!(survey.records.len(), 1);
        assert_eq!(survey.records[0].crate_root, "bk/crates/demo");
        assert_eq!(survey.records[0].path, "bk/crates/demo/examples/main.rs");
        assert!(survey.records[0].problems.is_empty());
        Ok(())
    }
}
