#![allow(dead_code)]
// TODO Reviewed ??-??-2026

// ANCHOR: example
//! Parse the arguments of a `cargo hello` subcommand and select packages.

use clap::Args;
use clap::Parser;

/// Cargo runs `cargo-hello hello <args>`, so the first argument is the name of
/// the subcommand. Declaring it as an enum variant makes clap skip it.
#[derive(Parser)]
#[command(name = "cargo", bin_name = "cargo")]
enum CargoCli {
    /// Say hello to the selected packages.
    Hello(HelloArgs),
}

/// The arguments of `cargo hello`.
#[derive(Args)]
struct HelloArgs {
    /// `-p`, `--workspace` and `--exclude`, as in cargo itself.
    #[command(flatten)]
    workspace: clap_cargo::Workspace,
    /// `--features`, `--all-features` and `--no-default-features`.
    #[command(flatten)]
    features: clap_cargo::Features,
    /// Path to `Cargo.toml`.
    #[arg(long, value_name = "PATH")]
    manifest_path: Option<std::path::PathBuf>,
}

/// Returns the names of the packages selected by the command-line flags.
fn selected_packages(args: &HelloArgs) -> anyhow::Result<Vec<String>> {
    let mut command = cargo_metadata::MetadataCommand::new();
    if let Some(path) = &args.manifest_path {
        command.manifest_path(path);
    }
    // Pass `--features` and friends to `cargo metadata`.
    args.features.forward_metadata(&mut command);
    let metadata = command.exec()?;
    // Apply `-p`, `--workspace` and `--exclude`.
    let (selected, _excluded) = args.workspace.partition_packages(&metadata);
    Ok(selected.iter().map(|p| p.name.to_string()).collect())
}

/// Runs the subcommand with the given arguments.
fn execute<I, T>(args: I) -> anyhow::Result<Vec<String>>
where
    I: IntoIterator<Item = T>,
    T: Into<std::ffi::OsString> + Clone,
{
    let CargoCli::Hello(args) = CargoCli::try_parse_from(args)?;
    selected_packages(&args)
}

/// Says hello to the packages of the current workspace.
pub fn run() -> anyhow::Result<()> {
    // In a real plugin, use `CargoCli::parse()` and print to `stdout`.
    // Here, build a throwaway workspace so that the example is self-contained.
    let dir = tempfile::tempdir()?;
    let manifest = dir.path().join("Cargo.toml");
    std::fs::write(
        &manifest,
        "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
    )?;
    std::fs::create_dir(dir.path().join("src"))?;
    std::fs::write(dir.path().join("src/lib.rs"), "")?;

    let manifest = manifest.display().to_string();
    for name in execute(["cargo", "hello", "--manifest-path", &manifest])? {
        println!("Hello, {name}!");
    }
    Ok(())
}
// ANCHOR_END: example

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_subcommand_name_is_skipped() -> anyhow::Result<()> {
        let CargoCli::Hello(args) = CargoCli::try_parse_from([
            "cargo",
            "hello",
            "--workspace",
            "--exclude",
            "other",
            "--all-features",
        ])?;
        assert!(args.workspace.workspace);
        assert_eq!(args.workspace.exclude, ["other"]);
        assert!(args.features.all_features);
        Ok(())
    }

    #[test]
    fn unknown_flags_are_rejected() {
        assert!(
            CargoCli::try_parse_from(["cargo", "hello", "--nope"]).is_err()
        );
    }

    #[test]
    fn selects_the_default_package() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        std::fs::create_dir(dir.path().join("src"))?;
        std::fs::write(dir.path().join("src/lib.rs"), "")?;
        let manifest = dir.path().join("Cargo.toml");
        std::fs::write(
            &manifest,
            "[package]\nname = \"demo\"\nversion = \"0.1.0\"\nedition = \"2021\"\n",
        )?;
        let manifest = manifest.display().to_string();
        let names = execute(["cargo", "hello", "--manifest-path", &manifest])?;
        assert_eq!(names, ["demo"]);
        let none = execute([
            "cargo",
            "hello",
            "--manifest-path",
            &manifest,
            "--workspace",
            "--exclude",
            "demo",
        ])?;
        assert!(none.is_empty());
        Ok(())
    }

    #[test]
    fn run_succeeds() -> anyhow::Result<()> {
        run()
    }
}
