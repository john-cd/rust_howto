//! Connects a `rust_howto` checkout to its external Cargo output directories.
//!
//! TODO Reviewed ??-??-2026.

use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
use std::process::ExitCode;
use std::process::Output;

use anyhow::Context as _;
use anyhow::Result;
use anyhow::bail;
use clap::Parser;
use clap::Subcommand;

/// Arguments for the build-setup command.
#[derive(Debug, Parser)]
#[command(version, about)]
struct Args {
    /// Repository checkout to configure. Defaults to the current directory.
    #[arg(long, global = true)]
    repo_root: Option<PathBuf>,

    /// Operation to perform. Defaults to `setup`.
    #[command(subcommand)]
    command: Option<Operation>,
}

/// Build environment operation selected by the caller.
#[derive(Debug, Subcommand)]
enum Operation {
    /// Create or verify this checkout's target, build, and sccache links.
    Setup {
        /// Override the target directory for a newly created worktree.
        #[arg(long)]
        target_dir: Option<PathBuf>,
        /// Override the intermediate build directory for a newly created worktree.
        #[arg(long)]
        build_dir: Option<PathBuf>,
    },
    /// Create a worktree in an unused repository slot and configure its output links.
    Worktree {
        /// One of the configured auxiliary worktree slots: a, b, or c.
        #[arg(long)]
        slot: String,
        /// Branch to check out or create in the requested slot.
        #[arg(long)]
        branch: String,
        /// Override the target directory for the new worktree.
        #[arg(long)]
        target_dir: Option<PathBuf>,
        /// Override the intermediate build directory for the new worktree.
        #[arg(long)]
        build_dir: Option<PathBuf>,
    },
    /// Restart the shared sccache daemon after confirming no Cargo or rustc process is running.
    RestartCache,
}

/// Runs the selected setup operation and reports actionable errors.
fn main() -> ExitCode {
    match run(Args::parse()) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("{error:#}");
            ExitCode::FAILURE
        }
    }
}

/// Applies the selected operation to the requested repository checkout.
///
/// # Errors
///
/// Returns an error when repository discovery, filesystem setup, or a child command fails.
fn run(args: Args) -> Result<()> {
    let repo_root = args
        .repo_root
        .map_or_else(env::current_dir, Ok)
        .context("could not determine the checkout path")?;
    let operation = args.command.unwrap_or(Operation::Setup {
        target_dir: None,
        build_dir: None,
    });

    match operation {
        Operation::Setup {
            target_dir,
            build_dir,
        } => setup_checkout(&repo_root, target_dir.as_deref(), build_dir.as_deref()),
        Operation::Worktree {
            slot,
            branch,
            target_dir,
            build_dir,
        } => {
            let checkout = create_worktree(&repo_root, &slot, &branch)?;
            setup_checkout(&checkout, target_dir.as_deref(), build_dir.as_deref())
        }
        Operation::RestartCache => restart_cache(&repo_root),
    }
}

/// Creates or verifies the three external-output links for one checkout.
///
/// # Errors
///
/// Returns an error if an existing path is not the expected link or a link cannot be created.
fn setup_checkout(
    repo_root: &Path,
    target_override: Option<&Path>,
    build_override: Option<&Path>,
) -> Result<()> {
    let repo_root = repo_root
        .canonicalize()
        .with_context(|| format!("could not resolve checkout {}", repo_root.display()))?;
    let slot = checkout_slot(&repo_root)?;
    let target_root = configured_root("CODE_TARGET_ROOT", "D:\\target", "cargo-target")?;
    let build_root = configured_root("CODE_BUILD_ROOT", "D:\\code_build", "cargo-build")?;
    let sccache_root = env::var_os("CODE_SCCACHE_ROOT")
        .or_else(|| env::var_os("SCCACHE_DIR"))
        .map_or_else(
            || default_cache_root("D:\\cache\\sccache", "sccache"),
            PathBuf::from,
        );

    let target = target_override
        .filter(|path| !path.as_os_str().is_empty())
        .map_or_else(
            || target_root.join("rust_howto").join(&slot),
            Path::to_path_buf,
        );
    let build = build_override
        .filter(|path| !path.as_os_str().is_empty())
        .map_or_else(
            || build_root.join("rust_howto").join(&slot),
            Path::to_path_buf,
        );
    let sccache = sccache_root;

    for (name, link, destination) in [
        ("target", repo_root.join("target"), target),
        ("build", repo_root.join("build"), build),
        (".sccache", repo_root.join(".sccache"), sccache),
    ] {
        let (destination, link_exists) = prepare_output_directory(&link, &destination)?;
        if !link_exists {
            connect_link(&link, &destination)
                .with_context(|| format!("could not configure {name}"))?;
        }
        println!("{} -> {}", link.display(), destination.display());
    }

    Ok(())
}

/// Returns the shared cache root selected by the environment or platform defaults.
fn configured_root(variable: &str, windows_default: &str, unix_name: &str) -> Result<PathBuf> {
    if let Some(value) = env::var_os(variable) {
        return Ok(PathBuf::from(value));
    }
    if cfg!(windows) {
        return Ok(PathBuf::from(windows_default));
    }
    Ok(default_cache_root(windows_default, unix_name))
}

/// Builds a cache directory under the configured XDG cache root or the user's cache folder.
fn default_cache_root(windows_default: &str, child: &str) -> PathBuf {
    if cfg!(windows) {
        return PathBuf::from(windows_default);
    }
    let cache = env::var_os("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".cache")))
        .unwrap_or_else(|| PathBuf::from(".cache"));
    cache.join(child)
}

/// Requires a destination path to be absolute before creating an external link.
fn ensure_absolute(path: &Path) -> Result<()> {
    if !path.is_absolute() {
        bail!("expected an absolute output path: {}", path.display());
    }
    Ok(())
}

/// Checks an existing link before creating its destination, then returns the resolved destination.
///
/// # Errors
///
/// Returns an error if the destination is relative, an existing link is unexpected, or filesystem
/// inspection and creation fail.
fn prepare_output_directory(link: &Path, destination: &Path) -> Result<(PathBuf, bool)> {
    ensure_absolute(destination)?;
    let link_exists = match fs::symlink_metadata(link) {
        Ok(_) => {
            let unresolved_destination = canonicalize_if_present(destination)?;
            if !existing_link_matches(link, &unresolved_destination)? {
                bail!(
                    "preserving existing {}; expected a link to {}",
                    link.display(),
                    destination.display()
                );
            }
            true
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => false,
        Err(error) => {
            return Err(error).with_context(|| format!("could not inspect {}", link.display()));
        }
    };
    fs::create_dir_all(destination).with_context(|| {
        format!(
            "could not create output directory {}",
            destination.display()
        )
    })?;
    let destination = destination.canonicalize().with_context(|| {
        format!(
            "could not resolve output directory {}",
            destination.display()
        )
    })?;
    if link_exists && !existing_link_matches(link, &destination)? {
        bail!(
            "existing {} link changed while preparing the output directory",
            link.display()
        );
    }
    Ok((destination, link_exists))
}

/// Resolves an existing path or returns its absolute spelling when it does not exist yet.
fn canonicalize_if_present(path: &Path) -> Result<PathBuf> {
    match path.canonicalize() {
        Ok(path) => Ok(path),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(path.to_path_buf()),
        Err(error) => Err(error)
            .with_context(|| format!("could not resolve output directory {}", path.display())),
    }
}

/// Returns the auxiliary slot name or `main` for the primary checkout.
fn checkout_slot(repo_root: &Path) -> Result<String> {
    let parent = repo_root.parent();
    if parent.is_some_and(|path| path.file_name() == Some(OsStr::new(".worktrees"))) {
        let slot = repo_root
            .file_name()
            .and_then(OsStr::to_str)
            .context("worktree path has no valid slot name")?;
        if !matches!(slot, "a" | "b" | "c") {
            bail!("use only .worktrees/a, .worktrees/b, or .worktrees/c");
        }
        return Ok(slot.to_owned());
    }
    Ok("main".to_owned())
}

/// Finds the repository root shared by linked worktrees.
fn main_repository_root(repo_root: &Path) -> Result<PathBuf> {
    let output = run_command(
        Command::new("git").arg("-C").arg(repo_root).args([
            "rev-parse",
            "--path-format=absolute",
            "--git-common-dir",
        ]),
        "git rev-parse --git-common-dir",
    )?;
    let common_dir = PathBuf::from(
        String::from_utf8(output.stdout)
            .context("Git returned a non-UTF-8 path")?
            .trim(),
    );
    common_dir
        .parent()
        .map(Path::to_path_buf)
        .context("shared Git directory has no parent repository")
}

/// Creates a directory symlink on Unix or an NTFS junction on Windows.
fn connect_link(link: &Path, destination: &Path) -> Result<()> {
    match fs::symlink_metadata(link) {
        Ok(_) => {
            if !existing_link_matches(link, destination)? {
                bail!(
                    "preserving existing {}; expected a link to {}",
                    link.display(),
                    destination.display()
                );
            }
            return Ok(());
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).with_context(|| format!("could not inspect {}", link.display()));
        }
    }

    if cfg!(windows) {
        create_windows_junction(destination, link)
    } else {
        create_unix_symlink(destination, link)
    }
}

/// Resolves an existing Unix link target relative to the link's parent directory.
fn normalize_link_target(target: &Path, parent: &Path) -> Result<PathBuf> {
    let path = if target.is_absolute() {
        target.to_path_buf()
    } else {
        parent.join(target)
    };
    canonicalize_if_present(&path)
        .with_context(|| format!("could not resolve link target {}", path.display()))
}

/// Verifies that the existing path is the expected Windows junction or Unix symlink.
fn existing_link_matches(link: &Path, destination: &Path) -> Result<bool> {
    let metadata = match fs::symlink_metadata(link) {
        Ok(metadata) => metadata,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(false),
        Err(error) => {
            return Err(error).with_context(|| format!("could not inspect {}", link.display()));
        }
    };
    if cfg!(windows) {
        return existing_windows_junction_matches(link, destination);
    }
    if metadata.file_type().is_symlink() {
        let target = fs::read_link(link)
            .with_context(|| format!("could not read existing link {}", link.display()))?;
        let target = normalize_link_target(&target, link.parent().unwrap_or(Path::new(".")))?;
        return Ok(paths_match(&target, destination));
    }
    Ok(false)
}

/// Compares paths according to the platform's path case behavior.
fn paths_match(left: &Path, right: &Path) -> bool {
    #[cfg(windows)]
    {
        left.to_string_lossy()
            .eq_ignore_ascii_case(&right.to_string_lossy())
    }
    #[cfg(not(windows))]
    {
        left == right
    }
}

/// Verifies an existing Windows junction against its expected target.
#[cfg(windows)]
fn existing_windows_junction_matches(link: &Path, destination: &Path) -> Result<bool> {
    if !junction::exists(link).with_context(|| {
        format!(
            "could not inspect existing Windows junction {}",
            link.display()
        )
    })? {
        return Ok(false);
    }
    let target = junction::get_target(link).with_context(|| {
        format!(
            "could not read existing Windows junction target {}",
            link.display()
        )
    })?;
    let target = canonicalize_if_present(&target)?;
    Ok(paths_match(&target, destination))
}

/// Reports that no Windows junction exists at the path.
#[cfg(not(windows))]
fn existing_windows_junction_matches(_link: &Path, _destination: &Path) -> Result<bool> {
    Ok(false)
}

/// Creates a Windows junction to the external output directory.
#[cfg(windows)]
fn create_windows_junction(destination: &Path, link: &Path) -> Result<()> {
    junction::create(destination, link).context("could not create NTFS junction")
}

/// Is unavailable off Windows.
#[cfg(not(windows))]
fn create_windows_junction(_destination: &Path, _link: &Path) -> Result<()> {
    bail!("Windows junction creation is unavailable on this platform")
}

/// Creates a Unix directory symlink to the external output directory.
#[cfg(unix)]
fn create_unix_symlink(destination: &Path, link: &Path) -> Result<()> {
    use std::os::unix::fs::symlink;

    symlink(destination, link)
        .with_context(|| format!("could not create symlink {}", link.display()))
}

/// Is unavailable on non-Unix platforms.
#[cfg(not(unix))]
fn create_unix_symlink(_destination: &Path, _link: &Path) -> Result<()> {
    bail!("Unix symlink creation is unavailable on this platform")
}

/// Creates a registered worktree for one supported auxiliary slot.
fn create_worktree(repo_root: &Path, slot: &str, branch: &str) -> Result<PathBuf> {
    if !matches!(slot, "a" | "b" | "c") {
        bail!("specify a worktree slot a, b, or c");
    }
    let main_root = main_repository_root(repo_root)?;
    let checkout = main_root.join(".worktrees").join(slot);
    match fs::symlink_metadata(&checkout) {
        Ok(_) => {
            bail!("slot {slot} already exists; inspect its status and reuse it without deleting it")
        }
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
        Err(error) => {
            return Err(error).with_context(|| {
                format!("could not inspect worktree slot {}", checkout.display())
            });
        }
    }
    run_command(
        Command::new("git").arg("-C").arg(&main_root).args([
            "check-ref-format",
            "--branch",
            branch,
        ]),
        "git check-ref-format",
    )?;
    let branch_exists = Command::new("git")
        .arg("-C")
        .arg(&main_root)
        .args(["show-ref", "--verify", "--quiet"])
        .arg(format!("refs/heads/{branch}"))
        .status()
        .context("could not inspect the requested branch")?
        .success();
    let mut command = Command::new("git");
    command.arg("-C").arg(&main_root).arg("worktree").arg("add");
    if branch_exists {
        command.arg(&checkout).arg(branch);
    } else {
        command.arg("-b").arg(branch).arg(&checkout).arg("HEAD");
    }
    run_command(&mut command, "git worktree add")?;
    Ok(checkout)
}

/// Restarts sccache after checking for active Rust builds.
fn restart_cache(repo_root: &Path) -> Result<()> {
    ensure_no_build_processes()?;
    let basedirs = sccache_base_dirs(repo_root)?;
    let value =
        env::join_paths(basedirs.iter()).context("could not encode sccache base directories")?;
    let _ = Command::new("sccache").arg("--stop-server").status();
    let mut start = Command::new("sccache");
    start
        .args(["--start-server"])
        .env("SCCACHE_BASEDIRS", &value)
        .env("SCCACHE_CACHE_SIZE", "45G");
    run_command(&mut start, "sccache --start-server")?;
    let mut stats = Command::new("sccache");
    stats
        .args(["--show-stats"])
        .env("SCCACHE_BASEDIRS", value)
        .env("SCCACHE_CACHE_SIZE", "45G");
    let output = run_command(&mut stats, "sccache --show-stats")?;
    print!("{}", String::from_utf8_lossy(&output.stdout));
    eprint!("{}", String::from_utf8_lossy(&output.stderr));
    Ok(())
}

/// Checks for running cargo and rustc processes before the shared daemon is restarted.
fn ensure_no_build_processes() -> Result<()> {
    #[cfg(windows)]
    {
        let output = run_command(
            Command::new("tasklist").args(["/FO", "CSV", "/NH"]),
            "tasklist",
        )?;
        let processes = String::from_utf8_lossy(&output.stdout).to_ascii_lowercase();
        if processes.contains("\"cargo.exe\"") || processes.contains("\"rustc.exe\"") {
            bail!("wait for running Cargo/rustc processes before restarting sccache");
        }
    }
    #[cfg(unix)]
    for process in ["cargo", "rustc"] {
        let status = Command::new("pgrep")
            .args(["-x", process])
            .status()
            .with_context(|| format!("could not check for running {process} processes"))?;
        if status.success() {
            bail!("wait for running Cargo/rustc processes before restarting sccache");
        }
        if status.code() != Some(1) {
            bail!("process check for {process} failed with {status}");
        }
    }
    Ok(())
}

/// Returns the repository's main checkout and registered letter worktrees for sccache.
fn sccache_base_dirs(repo_root: &Path) -> Result<Vec<PathBuf>> {
    let main_root = main_repository_root(repo_root)?;
    let output = run_command(
        Command::new("git")
            .arg("-C")
            .arg(&main_root)
            .args(["worktree", "list", "--porcelain"]),
        "git worktree list",
    )?;
    let listing =
        String::from_utf8(output.stdout).context("Git returned a non-UTF-8 worktree list")?;
    let allowed = main_root.join(".worktrees");
    let mut dirs = vec![main_root];
    for line in listing
        .lines()
        .filter_map(|line| line.strip_prefix("worktree "))
    {
        let path = PathBuf::from(line);
        let Some(parent) = path.parent() else {
            continue;
        };
        if parent == allowed
            && matches!(
                path.file_name().and_then(OsStr::to_str),
                Some("a" | "b" | "c")
            )
        {
            dirs.push(path);
        }
    }
    Ok(dirs)
}

/// Runs a child command and includes its rendered command name in failures.
fn run_command(command: &mut Command, label: &str) -> Result<Output> {
    let output = command
        .output()
        .with_context(|| format!("could not start {label}"))?;
    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        let stdout = String::from_utf8_lossy(&output.stdout);
        bail!(
            "{label} failed with {}\n{}{}",
            output.status,
            stdout,
            stderr
        );
    }
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Accepts only absolute output paths.
    #[test]
    fn output_path_must_be_absolute() -> Result<()> {
        let error = ensure_absolute(Path::new("relative/target"))
            .expect_err("relative output paths are rejected");
        assert!(error.to_string().contains("absolute"));
        let absolute = env::current_dir()
            .expect("current directory exists")
            .join("target");
        ensure_absolute(&absolute)?;
        Ok(())
    }

    /// Uses the stable main identity outside `.worktrees`.
    #[test]
    fn primary_checkout_uses_main_identity() -> Result<()> {
        let root = Path::new("D:/code/rust_howto");
        assert_eq!(checkout_slot(root)?, "main");
        Ok(())
    }

    /// Preserves the configured letter identities and rejects unknown slots.
    #[test]
    fn checkout_slot_validates_auxiliary_directory() -> Result<()> {
        assert_eq!(
            checkout_slot(Path::new("D:/code/rust_howto/.worktrees/b"))?,
            "b"
        );
        let error = checkout_slot(Path::new("D:/code/rust_howto/.worktrees/d"))
            .expect_err("unsupported worktree slots are rejected");
        assert!(error.to_string().contains(".worktrees/a"));
        Ok(())
    }

    /// Preserves a file at a link path and reports that it is not the expected output link.
    #[test]
    fn existing_file_is_preserved() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let link = dir.path().join("target");
        let destination = dir.path().join("new/external");
        fs::write(&link, "keep me")?;

        prepare_output_directory(&link, &destination)
            .expect_err("an existing file is not replaced with a link");
        assert!(!destination.exists());
        assert_eq!(fs::read_to_string(link)?, "keep me");
        Ok(())
    }

    /// Reuses a pre-existing link only when it resolves to the requested destination.
    #[cfg(unix)]
    #[test]
    fn existing_link_must_match_destination() -> Result<()> {
        let dir = tempfile::tempdir()?;
        let destination = dir.path().join("external");
        let other = dir.path().join("other");
        fs::create_dir(&destination)?;
        fs::create_dir(&other)?;
        let link = dir.path().join("target");

        create_unix_symlink(&other, &link)?;

        assert!(connect_link(&link, &destination).is_err());
        connect_link(&link, &other.canonicalize()?)?;
        Ok(())
    }
}
