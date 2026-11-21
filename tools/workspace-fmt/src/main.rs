//! Runs rustfmt over local Cargo targets in bounded batches.
//!
//! TODO Reviewed ??-??-2026.

use std::collections::BTreeMap;
use std::collections::BTreeSet;
use std::env;
use std::ffi::OsStr;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use clap::Parser;
use clap::Subcommand;
use serde::Deserialize;

/// Rustfmt workspace command-line arguments.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Cargo workspace directory. Defaults to the current directory.
    #[arg(long, global = true)]
    workspace_root: Option<PathBuf>,

    /// Format or check local Rust targets.
    #[command(subcommand)]
    command: FormatCommand,
}

/// Formatting operation requested by the caller.
#[derive(Debug, Subcommand)]
enum FormatCommand {
    /// Apply rustfmt to the selected workspace targets.
    Format,

    /// Check rustfmt without changing source files.
    Check,
}

/// The Cargo metadata fields used to select local Rust targets.
#[derive(Debug, Deserialize)]
struct CargoMetadata {
    /// Cargo's canonical workspace root.
    workspace_root: PathBuf,

    /// Packages and their declared targets.
    packages: Vec<CargoPackage>,
}

/// Package identity and target metadata emitted by Cargo.
#[derive(Debug, Deserialize)]
struct CargoPackage {
    /// `None` for local workspace or path packages and `Some` for registry packages.
    source: Option<String>,

    /// Edition used by rustfmt for the package.
    edition: String,

    /// Build targets declared by the package.
    targets: Vec<CargoTarget>,
}

/// Source path for one Cargo target.
#[derive(Debug, Deserialize)]
struct CargoTarget {
    /// Rust source file used as the target root.
    src_path: PathBuf,
}

/// Maximum target paths in one rustfmt child process.
const MAX_TARGETS_PER_BATCH: usize = 16;

/// Conservative Windows command-line estimate, below the 32,767 UTF-16 limit.
const WINDOWS_COMMAND_LINE_BUDGET: usize = 24_000;

/// Runs the requested formatting operation and reports a readable error.
fn main() -> ExitCode {
    match run() {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

/// Resolves Cargo metadata and formats each local Rust target in bounded batches.
///
/// # Errors
///
/// Returns an error if Cargo, rustup, metadata decoding, or rustfmt fails.
fn run() -> Result<()> {
    let args = Args::parse();
    let requested_root = args
        .workspace_root
        .map_or_else(env::current_dir, Ok)
        .context("could not determine the Cargo workspace directory")?;
    let requested_root = requested_root.canonicalize().with_context(|| {
        format!(
            "could not resolve Cargo workspace directory {}",
            requested_root.display()
        )
    })?;
    let metadata = cargo_metadata(&requested_root)?;
    let rustfmt = rustfmt_path(&requested_root)?;
    let check = matches!(args.command, FormatCommand::Check);
    let config = metadata.workspace_root.join("rustfmt.toml");
    let mut targets_by_edition = BTreeMap::<String, BTreeSet<PathBuf>>::new();

    for package in metadata.packages {
        if package.source.is_some() {
            continue;
        }
        let targets = targets_by_edition.entry(package.edition).or_default();
        targets.extend(
            package
                .targets
                .into_iter()
                .map(|target| target.src_path)
                .filter(|path| {
                    path.extension()
                        .is_some_and(|extension| extension == OsStr::new("rs"))
                }),
        );
    }

    let target_count = targets_by_edition
        .values()
        .map(BTreeSet::len)
        .sum::<usize>();
    if target_count == 0 {
        println!(
            "No local Rust targets found in {}",
            metadata.workspace_root.display()
        );
        return Ok(());
    }

    for (edition, targets) in targets_by_edition {
        let targets = targets.into_iter().collect::<Vec<_>>();
        println!(
            "{} {} local Rust targets in {} (edition {edition})",
            if check { "Checking" } else { "Formatting" },
            targets.len(),
            metadata.workspace_root.display()
        );
        let config_arg = config.is_file().then_some(config.as_path());
        let batches = target_batches(&rustfmt, &edition, config_arg, check, &targets)?;
        for batch in batches {
            let first_target = batch
                .first()
                .context("rustfmt target batch unexpectedly has no paths")?;
            let mut command = Command::new(&rustfmt);
            command.arg("--edition").arg(&edition);
            if let Some(config) = config_arg {
                command.arg("--config-path").arg(config);
            }
            if check {
                command.arg("--check");
            }
            command.args(&batch);
            let status = command.status().with_context(|| {
                format!("could not start rustfmt for {}", first_target.display())
            })?;
            if !status.success() {
                bail!(
                    "rustfmt failed with {status} while processing {}",
                    first_target.display()
                );
            }
        }
    }

    if check {
        println!("Rust formatting checks passed.");
    }
    Ok(())
}

/// Runs `cargo metadata` in the selected workspace and decodes its local targets.
fn cargo_metadata(workspace_root: &Path) -> Result<CargoMetadata> {
    let output = Command::new("cargo")
        .args(["metadata", "--locked", "--format-version", "1"])
        .current_dir(workspace_root)
        .output()
        .context("could not start cargo metadata")?;
    if !output.status.success() {
        bail!(
            "cargo metadata failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    serde_json::from_slice(&output.stdout).context("could not parse cargo metadata JSON")
}

/// Resolves the rustfmt executable selected by rustup and `RUSTFMT_TOOLCHAIN`.
fn rustfmt_path(workspace_root: &Path) -> Result<PathBuf> {
    let mut command = Command::new("rustup");
    command.args(["which", "rustfmt"]);
    if let Some(toolchain) = env::var_os("RUSTFMT_TOOLCHAIN") {
        command.arg("--toolchain").arg(toolchain);
    }
    let output = command
        .current_dir(workspace_root)
        .output()
        .context("could not start rustup to locate rustfmt")?;
    if !output.status.success() {
        bail!(
            "rustup which rustfmt failed with {}: {}",
            output.status,
            String::from_utf8_lossy(&output.stderr).trim()
        );
    }
    let path = String::from_utf8(output.stdout)
        .context("rustup returned a non-UTF-8 rustfmt path")?
        .trim_end_matches(['\r', '\n'])
        .to_owned();
    if path.is_empty() {
        bail!("rustup returned an empty rustfmt path");
    }
    Ok(PathBuf::from(path))
}

/// Splits target paths into batches within the Windows command-line budget.
fn target_batches(
    rustfmt: &Path,
    edition: &str,
    config: Option<&Path>,
    check: bool,
    targets: &[PathBuf],
) -> Result<Vec<Vec<PathBuf>>> {
    target_batches_for_platform(rustfmt, edition, config, check, targets, cfg!(windows))
}

/// Splits target paths using the selected platform's argument budget.
fn target_batches_for_platform(
    rustfmt: &Path,
    edition: &str,
    config: Option<&Path>,
    check: bool,
    targets: &[PathBuf],
    is_windows: bool,
) -> Result<Vec<Vec<PathBuf>>> {
    let mut fixed_arguments = vec![
        rustfmt.as_os_str(),
        OsStr::new("--edition"),
        OsStr::new(edition),
    ];
    if let Some(config) = config {
        fixed_arguments.extend([OsStr::new("--config-path"), config.as_os_str()]);
    }
    if check {
        fixed_arguments.push(OsStr::new("--check"));
    }
    let fixed_length = fixed_arguments
        .into_iter()
        .map(|argument| estimated_argument_length(argument, is_windows))
        .sum::<usize>();
    let budget = if is_windows {
        WINDOWS_COMMAND_LINE_BUDGET
    } else {
        usize::MAX
    };
    let mut batches = Vec::new();
    let mut batch = Vec::new();
    let mut command_length = fixed_length;

    for target in targets {
        let target_length = estimated_argument_length(target.as_os_str(), is_windows);
        if !batch.is_empty()
            && (batch.len() == MAX_TARGETS_PER_BATCH || command_length + target_length > budget)
        {
            batches.push(std::mem::take(&mut batch));
            command_length = fixed_length;
        }
        if fixed_length + target_length > budget {
            bail!(
                "rustfmt arguments exceed the Windows command-line budget for {}",
                target.display()
            );
        }
        command_length += target_length;
        batch.push(target.clone());
    }
    if !batch.is_empty() {
        batches.push(batch);
    }
    Ok(batches)
}

/// Estimates one argument's command-line space with a conservative quote allowance.
fn estimated_argument_length(argument: &OsStr, windows: bool) -> usize {
    if windows {
        argument.to_string_lossy().encode_utf16().count() + 4
    } else {
        argument.len() + 1
    }
}

/// Verifies rustfmt batching limits and Windows command-line budget handling.
#[cfg(test)]
mod tests {
    use super::*;

    /// Keeps batches within the per-process target count and preserves target order.
    #[test]
    fn target_batches_split_at_the_target_count() -> Result<()> {
        let targets = (0..17)
            .map(|index| PathBuf::from(format!("src/{index}.rs")))
            .collect::<Vec<_>>();

        let batches = target_batches(Path::new("rustfmt"), "2024", None, true, &targets)?;
        assert_eq!(
            batches.iter().map(Vec::len).collect::<Vec<_>>(),
            [MAX_TARGETS_PER_BATCH, 1]
        );
        assert_eq!(batches.into_iter().flatten().collect::<Vec<_>>(), targets);
        Ok(())
    }

    /// Splits Windows batches by command-line size and rejects a single oversized path.
    #[test]
    fn windows_batches_respect_the_command_line_budget() -> Result<()> {
        let targets = [
            PathBuf::from(format!("C:\\{}", "a".repeat(12_050))),
            PathBuf::from(format!("C:\\{}", "b".repeat(12_050))),
        ];
        let batches = target_batches_for_platform(
            Path::new("C:\\rustfmt.exe"),
            "2024",
            Some(Path::new("C:\\rustfmt.toml")),
            true,
            &targets,
            true,
        )?;
        assert_eq!(batches.len(), 2);
        assert!(batches.iter().all(|batch| batch.len() == 1));

        let oversized = [PathBuf::from(format!(
            "C:\\{}",
            "x".repeat(WINDOWS_COMMAND_LINE_BUDGET)
        ))];
        let error = target_batches_for_platform(
            Path::new("C:\\rustfmt.exe"),
            "2024",
            None,
            false,
            &oversized,
            true,
        )
        .expect_err("a single oversized Windows argument should fail");
        assert!(error.to_string().contains("Windows command-line budget"));
        Ok(())
    }
}
