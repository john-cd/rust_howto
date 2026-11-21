//! Synchronizes shared Cargo lint profiles and checks dependency-audit workflow coverage.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::ExitCode;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use clap::Parser;
use clap::Subcommand;
use serde_yaml_ng::Mapping;
use serde_yaml_ng::Value;

/// Command-line arguments for repository configuration checks.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Repository root to inspect. Defaults to this checkout.
    #[arg(long, global = true)]
    repo_root: Option<PathBuf>,

    /// Configuration operation to run.
    #[command(subcommand)]
    command: Command,
}

/// Configuration maintenance and validation commands.
#[derive(Debug, Subcommand)]
enum Command {
    /// Synchronize generated lint tables with the shared TOML profiles.
    SyncLints {
        /// Report drift without writing manifests; return a failing status when drift exists.
        #[arg(long)]
        check: bool,
    },
    /// Ensure the dependency-audit workflow covers each configured workspace and manifest.
    AuditWorkflowPaths,
}

/// A Cargo manifest and the shared lint profile that supplies its generated tables.
#[derive(Clone, Copy)]
struct LintTarget {
    manifest: &'static str,
    profile: &'static str,
    workspace: bool,
}

/// Manifests kept in sync with the shared lint profiles.
const LINT_TARGETS: [LintTarget; 5] = [
    LintTarget {
        manifest: "bk/crates/Cargo.toml",
        profile: "examples",
        workspace: true,
    },
    LintTarget {
        manifest: "later/crates/Cargo.toml",
        profile: "examples",
        workspace: true,
    },
    LintTarget {
        manifest: "xmpl/Cargo.toml",
        profile: "examples",
        workspace: true,
    },
    LintTarget {
        manifest: "tools/Cargo.toml",
        profile: "strict",
        workspace: true,
    },
    LintTarget {
        manifest: "publish/Cargo.toml",
        profile: "strict",
        workspace: false,
    },
];

/// A Cargo workspace and the policy file used to audit it.
const AUDITED_WORKSPACES: [(&str, &str); 12] = [
    ("bk/crates", "bk/crates/deny.toml"),
    ("bk/platform", "bk/platform/deny.toml"),
    (
        "bk/crates/cats/database/examples/query_builders_orms/toasty_validation",
        "bk/crates/deny.toml",
    ),
    (
        "bk/crates/cats/memory_management/allocator_selection",
        "bk/crates/deny.toml",
    ),
    ("later/crates", "later/crates/deny.toml"),
    (
        "bk/platform/cats/development_tools_ffi/python_extension",
        "bk/platform/deny.toml",
    ),
    ("bk/platform/cats/embedded", "bk/platform/deny.toml"),
    ("bk/platform/cats/gui/vizia", "bk/platform/deny.toml"),
    (
        "bk/platform/cats/wasm/examples/wasm_standalone_runtimes/components/guest",
        "bk/platform/deny.toml",
    ),
    ("publish", "bk/crates/deny.toml"),
    ("tools", "bk/crates/deny.toml"),
    ("xmpl", "bk/crates/deny.toml"),
];

/// Runs the selected check and prints a readable top-level error on failure.
fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(code) => code,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

/// Selects the repository root and invokes the requested operation.
///
/// # Errors
///
/// Returns an error if the selected operation cannot read or update its inputs.
fn run(args: Args) -> Result<ExitCode> {
    let root = args
        .repo_root
        .map_or_else(default_repository_root, Ok)
        .context("could not determine the repository root")?;
    match args.command {
        Command::SyncLints { check } => sync_lints(&root, check),
        Command::AuditWorkflowPaths => audit_workflow_paths(&root),
    }
}

/// Returns the root of the checkout containing this package.
fn default_repository_root() -> Result<PathBuf> {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .map(Path::to_path_buf)
        .context("repo-audit is not nested in a repository tools directory")
}

/// Synchronizes lint tables or reports drift without modifying files.
///
/// # Errors
///
/// Returns an error if a profile or manifest is missing, malformed, or cannot be written.
fn sync_lints(root: &Path, check: bool) -> Result<ExitCode> {
    let mut drifted = Vec::new();
    for target in LINT_TARGETS {
        let manifest_path = root.join(target.manifest);
        let profile_path = root
            .join("scripts/lints")
            .join(format!("{}.toml", target.profile));
        let current = fs::read_to_string(&manifest_path)
            .with_context(|| format!("could not read {}", manifest_path.display()))?;
        let profile = fs::read_to_string(&profile_path)
            .with_context(|| format!("could not read {}", profile_path.display()))?;
        let line_ending = if current.contains("\r\n") {
            "\r\n"
        } else {
            "\n"
        };
        let normalized = current.replace("\r\n", "\n");
        let updated = render_lint_tables(&normalized, &profile, target.workspace)
            .with_context(|| {
                format!(
                    "could not render {} from {}",
                    target.manifest, target.profile
                )
            })?
            .replace('\n', line_ending);
        if updated != current {
            drifted.push(target.manifest);
            if !check {
                fs::write(&manifest_path, updated)
                    .with_context(|| format!("could not update {}", manifest_path.display()))?;
            }
        }
    }
    if drifted.is_empty() {
        println!("lint tables match the shared profiles");
        return Ok(ExitCode::SUCCESS);
    }

    if check {
        println!(
            "drifted from the shared lint profiles: {}",
            drifted.join(", ")
        );
        Ok(ExitCode::FAILURE)
    } else {
        println!(
            "updated to match the shared lint profiles: {}",
            drifted.join(", ")
        );
        Ok(ExitCode::SUCCESS)
    }
}

/// Renders the generated lint table block while preserving the surrounding manifest text.
///
/// TOML parsing validates both inputs and confirms the expected lint table exists. The source
/// replacement keeps comments and line ordering byte-for-byte, which a TOML serializer would not.
///
/// # Errors
///
/// Returns an error if the TOML is malformed or does not contain a Rust lint table.
fn render_lint_tables(manifest: &str, profile: &str, workspace: bool) -> Result<String> {
    let manifest_toml: toml::Value =
        toml::from_str(manifest).context("manifest is not valid TOML")?;
    let profile_toml: toml::Value =
        toml::from_str(profile).context("lint profile is not valid TOML")?;
    let manifest_lints = lint_tables(&manifest_toml, workspace);
    let profile_lints = lint_tables(&profile_toml, true);
    if manifest_lints.is_none() {
        bail!("no Rust lint table found in manifest");
    }
    if profile_lints.is_none() {
        bail!("no Rust lint table found in shared profile");
    }

    let lint_header = if workspace {
        "[workspace.lints.rust]"
    } else {
        "[lints.rust]"
    };
    let start =
        line_starting_with(manifest, lint_header).context("no Rust lint table header found")?;
    let lint_end = next_non_lint_table(manifest, start);
    let tail_start = lint_end.map_or(manifest.len(), |end| {
        trailing_comment_start(manifest, start, end)
    });
    let profile_header = "[workspace.lints.rust]";
    let profile_start = line_starting_with(profile, profile_header)
        .context("no Rust lint table header found in shared profile")?;
    let profile_body = profile
        .get(profile_start..)
        .context("lint profile table offset is not on a character boundary")?;
    let mut body = profile_body.trim_end_matches('\n').to_owned();
    body.push('\n');
    if !workspace {
        body = body.replace("[workspace.lints.", "[lints.");
    }

    let mut rendered = String::with_capacity(manifest.len() + body.len());
    let prefix = manifest
        .get(..start)
        .context("manifest table offset is not on a character boundary")?;
    rendered.push_str(prefix);
    rendered.push_str(&body);
    if lint_end.is_some() {
        let tail = manifest
            .get(tail_start..)
            .context("manifest tail offset is not on a character boundary")?;
        rendered.push_str(tail);
    }
    Ok(rendered)
}

/// Finds the Rust lint table within a parsed TOML document.
fn lint_tables(document: &toml::Value, workspace: bool) -> Option<&toml::value::Table> {
    let lints = if workspace {
        document.get("workspace")?.get("lints")?
    } else {
        document.get("lints")?
    };
    lints.get("rust")?.as_table()
}

/// Returns the byte offset of a line whose content exactly equals the requested table header.
fn line_starting_with(text: &str, expected: &str) -> Option<usize> {
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if line.trim_end_matches(['\r', '\n']) == expected {
            return Some(offset);
        }
        offset += line.len();
    }
    None
}

/// Finds the first following table outside the generated lint namespace.
fn next_non_lint_table(text: &str, start: usize) -> Option<usize> {
    let mut offset = start;
    let mut after_header = false;
    for line in text.get(start..)?.split_inclusive('\n') {
        let content = line.trim_end_matches(['\r', '\n']);
        if after_header
            && content.starts_with('[')
            && !(content.starts_with("[workspace.lints.") || content.starts_with("[lints."))
        {
            return Some(offset);
        }
        if content.starts_with('[') {
            after_header = true;
        }
        offset += line.len();
    }
    None
}

/// Includes a trailing block of comments with the next table, matching the previous generator.
fn trailing_comment_start(text: &str, start: usize, end: usize) -> usize {
    let mut offset = start;
    let mut trailing_comments = None;
    let mut preceding_blank = None;
    let Some(section) = text.get(start..end) else {
        return end;
    };
    for line in section.split_inclusive('\n') {
        if line.starts_with('#') {
            trailing_comments.get_or_insert(preceding_blank.unwrap_or(offset));
            preceding_blank = None;
        } else if line.trim().is_empty() {
            if trailing_comments.is_some() {
                trailing_comments = Some(preceding_blank.unwrap_or(offset));
            }
            preceding_blank.get_or_insert(offset);
        } else {
            trailing_comments = None;
            preceding_blank = None;
        }
        offset += line.len();
    }
    trailing_comments.unwrap_or(end)
}

/// Checks that each audit trigger covers the configured workspace manifests and policy files.
///
/// # Errors
///
/// Returns an error if the workflow cannot be read or parsed as YAML.
fn audit_workflow_paths(root: &Path) -> Result<ExitCode> {
    let workflow_path = root.join(".github/workflows/audit_dependencies.yml");
    let workflow = fs::read_to_string(&workflow_path)
        .with_context(|| format!("could not read {}", workflow_path.display()))?;
    let paths = trigger_paths(&workflow)
        .with_context(|| format!("could not parse {}", workflow_path.display()))?;
    let problems = missing_workflow_paths(&paths);
    if problems.is_empty() {
        println!("dependency-audit workflow covers all configured workspaces");
        return Ok(ExitCode::SUCCESS);
    }
    for problem in problems {
        eprintln!("{problem}");
    }
    Ok(ExitCode::FAILURE)
}

/// Parses trigger path filters from the workflow's `push` and `pull_request` event mappings.
///
/// # Errors
///
/// Returns an error when the YAML is malformed or a `paths` entry is not a string.
fn trigger_paths(workflow: &str) -> Result<BTreeMap<String, BTreeSet<String>>> {
    let document: Value =
        serde_yaml_ng::from_str(workflow).context("workflow is not valid YAML")?;
    let root = document
        .as_mapping()
        .context("workflow root must be a YAML mapping")?;
    let triggers = mapping_value(root, "on")
        .or_else(|| root.get(Value::Bool(true)))
        .and_then(Value::as_mapping)
        .context("workflow must define an `on` mapping")?;
    let mut result = BTreeMap::new();
    for event in ["push", "pull_request"] {
        let Some(event_value) = mapping_value(triggers, event) else {
            result.insert(event.to_owned(), BTreeSet::new());
            continue;
        };
        let Some(paths) = event_value
            .as_mapping()
            .and_then(|mapping| mapping_value(mapping, "paths"))
        else {
            result.insert(event.to_owned(), BTreeSet::new());
            continue;
        };
        let sequence = paths
            .as_sequence()
            .with_context(|| format!("{event}.paths must be a YAML sequence"))?;
        let mut event_paths = BTreeSet::new();
        for path in sequence {
            let path = path
                .as_str()
                .with_context(|| format!("{event}.paths entries must be strings"))?;
            event_paths.insert(path.to_owned());
        }
        result.insert(event.to_owned(), event_paths);
    }
    Ok(result)
}

/// Looks up a string-keyed YAML mapping entry.
fn mapping_value<'a>(mapping: &'a Mapping, key: &str) -> Option<&'a Value> {
    mapping.get(Value::String(key.to_owned()))
}

/// Returns one diagnostic for every required path missing from either trigger.
fn missing_workflow_paths(paths: &BTreeMap<String, BTreeSet<String>>) -> Vec<String> {
    let workflow = ".github/workflows/audit_dependencies.yml";
    let mut required = BTreeSet::from([workflow.to_owned()]);
    for (workspace, policy) in AUDITED_WORKSPACES {
        required.extend([
            format!("{workspace}/Cargo.lock"),
            format!("{workspace}/Cargo.toml"),
            format!("{workspace}/**/Cargo.toml"),
            policy.to_owned(),
        ]);
    }
    let mut problems = Vec::new();
    for event in ["push", "pull_request"] {
        let listed = paths.get(event);
        for required_path in &required {
            if !listed.is_some_and(|event_paths| event_paths.contains(required_path)) {
                problems.push(format!("{event}: missing {required_path}"));
            }
        }
    }
    problems
}

/// Exercises lint block replacement and workflow coverage with stable fixtures.
#[cfg(test)]
mod tests {
    use super::*;

    const AUDIT_WORKFLOW: &str = include_str!("../../../.github/workflows/audit_dependencies.yml");

    /// Replaces the Rust lint tables while retaining comments before the next manifest table.
    #[test]
    fn lint_rendering_preserves_following_comments() -> Result<()> {
        let manifest = "# manifest\n\n[workspace.lints.rust]\nold = 'warn'\n\n# package settings\n[workspace.package]\nversion = '1.0.0'\n";
        let profile = "# shared profile\n[workspace.lints.rust]\nunsafe_code = 'deny'\n[workspace.lints.clippy]\nall = 'warn'\n";

        let rendered = render_lint_tables(manifest, profile, true)?;
        assert!(rendered.contains("unsafe_code = 'deny'"));
        assert!(!rendered.contains("old = 'warn'"));
        assert!(rendered.contains("\n\n# package settings\n[workspace.package]"));
        Ok(())
    }

    /// Rewrites workspace lint table headers for a single-package manifest.
    #[test]
    fn lint_rendering_supports_single_package_manifests() -> Result<()> {
        let manifest = "[lints.rust]\nold = 'warn'\n\n[package]\nname = 'example'\n";
        let profile = "[workspace.lints.rust]\nunsafe_code = 'deny'\n[workspace.lints.clippy]\nall = 'warn'\n";

        let rendered = render_lint_tables(manifest, profile, false)?;
        assert!(rendered.contains("[lints.rust]"));
        assert!(rendered.contains("[lints.clippy]"));
        assert!(!rendered.contains("workspace.lints"));
        Ok(())
    }

    /// Checks that `--check` reports drift without writing, and synchronization keeps CRLF files.
    #[test]
    fn sync_check_does_not_write_and_sync_preserves_crlf() -> Result<()> {
        let root = tempfile::tempdir()?;
        let profile_dir = root.path().join("scripts/lints");
        fs::create_dir_all(&profile_dir)?;
        fs::write(
            profile_dir.join("examples.toml"),
            "[workspace.lints.rust]\nunsafe_code = 'allow'\n",
        )?;
        fs::write(
            profile_dir.join("strict.toml"),
            "[workspace.lints.rust]\nunsafe_code = 'deny'\n",
        )?;

        let mut originals = BTreeMap::new();
        for target in LINT_TARGETS {
            let manifest_path = root.path().join(target.manifest);
            fs::create_dir_all(manifest_path.parent().context("manifest has a parent")?)?;
            let lint_header = if target.workspace {
                "[workspace.lints.rust]"
            } else {
                "[lints.rust]"
            };
            let following_table = if target.workspace {
                "[workspace.package]"
            } else {
                "[package]"
            };
            let text = format!(
                "{lint_header}\r\nold = 'warn'\r\n\r\n{following_table}\r\nname = 'fixture'\r\n"
            );
            fs::write(&manifest_path, &text)?;
            originals.insert(target.manifest, text);
        }

        assert_eq!(sync_lints(root.path(), true)?, ExitCode::FAILURE);
        for (relative, original) in &originals {
            assert_eq!(fs::read_to_string(root.path().join(relative))?, *original);
        }

        assert_eq!(sync_lints(root.path(), false)?, ExitCode::SUCCESS);
        assert_eq!(sync_lints(root.path(), true)?, ExitCode::SUCCESS);
        for target in LINT_TARGETS {
            let updated = fs::read(root.path().join(target.manifest))?;
            assert!(updated.windows(2).any(|pair| pair == b"\r\n"));
            assert!(!updated.windows(2).any(|pair| pair == b"\n\n"));
        }
        Ok(())
    }

    /// Parses the committed workflow and verifies its configured path coverage.
    #[test]
    fn committed_audit_workflow_covers_every_configured_path() -> Result<()> {
        let paths = trigger_paths(AUDIT_WORKFLOW)?;
        assert!(missing_workflow_paths(&paths).is_empty());
        Ok(())
    }

    /// Reports a missing top-level workspace lockfile for both audited events.
    #[test]
    fn missing_lockfile_is_reported_for_each_event() -> Result<()> {
        let mut paths = trigger_paths(AUDIT_WORKFLOW)?;
        for event in ["push", "pull_request"] {
            paths
                .get_mut(event)
                .context("event is parsed")?
                .remove("xmpl/Cargo.lock");
        }
        let problems = missing_workflow_paths(&paths);
        assert!(problems.contains(&"push: missing xmpl/Cargo.lock".to_owned()));
        assert!(problems.contains(&"pull_request: missing xmpl/Cargo.lock".to_owned()));
        Ok(())
    }

    /// Reports a missing nested workspace lockfile for both audited events.
    #[test]
    fn missing_nested_lockfile_is_reported_for_each_event() -> Result<()> {
        let nested_lockfile = concat!(
            "bk/platform/cats/wasm/examples/wasm_standalone_runtimes/",
            "components/guest/Cargo.lock"
        );
        let mut paths = trigger_paths(AUDIT_WORKFLOW)?;
        for event in ["push", "pull_request"] {
            paths
                .get_mut(event)
                .context("event is parsed")?
                .remove(nested_lockfile);
        }
        let problems = missing_workflow_paths(&paths);
        assert!(problems.contains(&format!("push: missing {nested_lockfile}")));
        assert!(problems.contains(&format!("pull_request: missing {nested_lockfile}")));
        Ok(())
    }
}
