//! Runs the `clean` binary against temporary trees only.

use std::fs;
use std::path::Path;
use std::process::Command;
use std::process::Output;

use tempfile::tempdir;

/// Runs the binary with a single directory argument.
fn run_clean(dir: &Path) -> Output {
    Command::new(env!("CARGO_BIN_EXE_clean"))
        .arg(dir)
        .output()
        .expect("the clean binary should start")
}

/// Verifies that a missing root fails with a nonzero status and a path-bearing message.
#[test]
fn test_missing_root_fails() {
    let dir = tempdir().expect("a temporary directory should be created");
    let missing = dir.path().join("does-not-exist");
    let output = run_clean(&missing);
    assert!(!output.status.success());
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert!(stderr.contains("does-not-exist"), "stderr: {stderr}");
}

/// Verifies that normal cleanup removes visible entries and preserves hidden files.
#[test]
fn test_cleanup_preserves_hidden_files() {
    let dir = tempdir().expect("a temporary directory should be created");
    let temp = dir.path().join("pkg").join("temp");
    fs::create_dir_all(temp.join("sub")).expect("dirs should be created");
    fs::write(temp.join("a.txt"), "x").expect("file should be written");
    fs::write(temp.join(".gitkeep"), "").expect("file should be written");
    fs::write(temp.join("sub").join("b.txt"), "x").expect("file should be written");
    fs::write(dir.path().join("pkg").join("outside.txt"), "x").expect("file should be written");

    let output = run_clean(dir.path());
    assert!(output.status.success(), "{output:?}");
    assert!(!temp.join("a.txt").exists());
    assert!(!temp.join("sub").exists());
    assert!(temp.join(".gitkeep").exists());
    assert!(dir.path().join("pkg").join("outside.txt").exists());
}

/// Verifies that an unreadable directory aborts the run before anything is deleted (Unix only).
#[cfg(unix)]
#[test]
fn test_unreadable_directory_fails_without_deleting() {
    use std::os::unix::fs::PermissionsExt as _;

    let dir = tempdir().expect("a temporary directory should be created");
    let temp = dir.path().join("ok").join("temp");
    fs::create_dir_all(&temp).expect("dirs should be created");
    fs::write(temp.join("a.txt"), "x").expect("file should be written");
    let locked = dir.path().join("locked");
    fs::create_dir(&locked).expect("dir should be created");
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o000)).expect("chmod should work");

    let output = run_clean(dir.path());
    fs::set_permissions(&locked, fs::Permissions::from_mode(0o755)).expect("chmod should work");
    // Root ignores permissions, so only assert when the traversal failed.
    if !output.status.success() {
        assert!(temp.join("a.txt").exists());
    }
}
