// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Counts how often each file changed in the last commits with `gix`,
//! a pure-Rust implementation of Git (no `git` executable or `libgit2`).
//!
//! Code churn is how often files change. Files that change very often tend to
//! be where bugs concentrate, so counting commits per file from Git history
//! helps decide what to refactor or test.

use std::collections::BTreeMap;
use std::path::Path;

use gix::bstr::ByteSlice;
use gix::object::tree::diff::Action;

/// Counts the commits (among the last `limit`, first parent only) that
/// touched each path. Shallow boundary commits have no available parent diff.
fn churn(repo: &gix::Repository, limit: usize) -> anyhow::Result<BTreeMap<String, usize>> {
    let mut counts = BTreeMap::new();
    let head = repo.head_commit()?;
    let shallow_commits = repo.shallow_commits()?;
    for info in head.ancestors().first_parent_only().all()?.take(limit) {
        let commit = info?.object()?;
        if shallow_commits
            .as_ref()
            .is_some_and(|ids| ids.binary_search(&commit.id).is_ok())
        {
            continue;
        }

        let tree = commit.tree()?;
        // The first commit has no parent: compare with the empty tree.
        let parent_tree = match commit.parent_ids().next() {
            Some(id) => id.object()?.into_commit().tree()?,
            None => repo.empty_tree(),
        };
        parent_tree
            .changes()?
            .for_each_to_obtain_tree(&tree, |change| {
                let path = change.location().to_str_lossy().into_owned();
                *counts.entry(path).or_insert(0) += 1;
                Ok(Action::Continue(()))
            })?;
    }
    Ok(counts)
}

/// Finds the repository containing `path` and counts recent path changes.
fn churn_at(path: &Path, limit: usize) -> anyhow::Result<BTreeMap<String, usize>> {
    let repo = gix::discover(path)?;
    churn(&repo, limit)
}

/// Prints the most frequently changed paths in the current checkout.
fn main() -> anyhow::Result<()> {
    let counts = churn_at(Path::new(env!("CARGO_MANIFEST_DIR")), 20)?;
    let mut ranked: Vec<_> = counts.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (path, count) in ranked.into_iter().take(5) {
        println!("{count:>3} {path}");
    }
    Ok(())
}
// ANCHOR_END: example

/// Prints the most frequently changed paths in the current checkout.
///
/// # Errors
///
/// Returns an error if a step of the example fails.
pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips
    // `clippy::main_recursion`.
    let entry = main;
    entry()
}

/// Exercises history traversal with a temporary repository.
#[cfg(test)]
mod tests {
    use std::process::Command;

    use super::*;

    /// Runs a Git fixture command and surfaces its diagnostic on failure.
    fn git(dir: &Path, args: &[&str]) -> anyhow::Result<()> {
        let output = Command::new("git")
            .current_dir(dir)
            .args([
                "-c",
                "user.name=Test Author",
                "-c",
                "user.email=test@example.invalid",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .output()?;
        anyhow::ensure!(
            output.status.success(),
            "git {args:?} failed: {}",
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(())
    }

    /// Counts changes in full history and skips the boundary in a shallow clone.
    #[test]
    fn counts_changes_up_to_the_available_history() -> anyhow::Result<()> {
        let temp = tempfile::tempdir()?;
        let full = temp.path().join("full");
        git(temp.path(), &["init", "--quiet", "full"])?;
        for content in ["first", "second"] {
            std::fs::write(full.join("notes.txt"), content)?;
            git(&full, &["add", "--", "notes.txt"])?;
            git(&full, &["commit", "--quiet", "-m", content])?;
        }
        assert_eq!(
            churn_at(&full, 20)?,
            BTreeMap::from([("notes.txt".into(), 2)])
        );

        git(
            temp.path(),
            &[
                "clone",
                "--quiet",
                "--depth=1",
                "--no-local",
                "full",
                "shallow",
            ],
        )?;
        assert!(churn_at(&temp.path().join("shallow"), 20)?.is_empty());
        Ok(())
    }

    /// Reports a clear failure for a temporary directory outside any repository.
    #[test]
    fn not_a_repository_is_an_error() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        assert!(churn_at(dir.path(), 1).is_err());
        Ok(())
    }
}
