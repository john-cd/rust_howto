// TODO Reviewed ??-??-2026

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

fn main() -> anyhow::Result<()> {
    // Find the repository that contains this source file.
    let repo = match gix::discover(Path::new(env!("CARGO_MANIFEST_DIR"))) {
        Ok(repo) => repo,
        Err(e) => {
            println!("Not in a Git repository: {e}");
            return Ok(());
        }
    };
    let counts = churn(&repo, 20)?;
    let mut ranked: Vec<_> = counts.iter().collect();
    ranked.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    for (path, count) in ranked.into_iter().take(5) {
        println!("{count:>3} {path}");
    }
    Ok(())
}
// ANCHOR_END: example

/// Runs the example.
#[allow(clippy::main_recursion)]
pub fn run() -> anyhow::Result<()> {
    main()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore = "needs a Git checkout with history"]
    fn test_main() -> anyhow::Result<()> {
        main()
    }

    #[test]
    fn not_a_repository_is_an_error() {
        let dir = std::env::temp_dir().join("not_a_git_repository_for_gix");
        std::fs::create_dir_all(&dir).expect("create dir");
        assert!(gix::open(&dir).is_err());
    }
}
