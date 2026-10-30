// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Counts how often each file changed in the last commits with `gix`,
//! a pure-Rust implementation of Git (no `git` executable or `libgit2`).

use std::collections::BTreeMap;
use std::path::Path;

use gix::bstr::ByteSlice;
use gix::object::tree::diff::Action;

/// Counts the commits (among the last `limit`, first parent only) that
/// touched each path.
fn churn(
    repo: &gix::Repository,
    limit: usize,
) -> anyhow::Result<BTreeMap<String, usize>> {
    let mut counts = BTreeMap::new();
    let head = repo.head_commit()?;
    for info in head.ancestors().first_parent_only().all()?.take(limit) {
        let commit = info?.object()?;
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
fn churn_at(
    path: &Path,
    limit: usize,
) -> anyhow::Result<BTreeMap<String, usize>> {
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

pub fn run() -> anyhow::Result<()> {
    // Call through a binding: calling `main` directly trips `clippy::main_recursion`.
    let entry = main;
    entry()
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Counts recent file changes in the checkout containing this example.
    #[test]
    fn counts_changes_in_the_example_checkout() -> anyhow::Result<()> {
        main()
    }

    /// Reports a clear failure for a temporary directory outside any
    /// repository.
    #[test]
    fn not_a_repository_is_an_error() -> anyhow::Result<()> {
        let dir = tempfile::tempdir()?;
        assert!(churn_at(dir.path(), 1).is_err());
        Ok(())
    }
}
