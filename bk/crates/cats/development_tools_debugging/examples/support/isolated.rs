//! Test helper: runs a test body in a fresh process.
//!
//! Several examples install a process-wide logger or tracing subscriber, which
//! can be done only once per process. `cargo test` runs every test of an
//! example binary in one process, so those tests re-run themselves in a child
//! process. `cargo nextest` already starts a process per test; the child is a
//! cheap, harmless repeat there.

use std::process::Command;

/// Environment variable that marks the child process.
const CHILD_MARKER: &str = "EXAMPLE_TEST_ISOLATED_CHILD";

/// Runs `body` in a dedicated process.
///
/// `module_path` is `module_path!()` of the calling test module, and `name` is
/// the test function's name. In the parent process, the test re-executes
/// itself with the marker set and asserts that the child succeeded; in the
/// child, `body` runs.
pub fn run(module_path: &str, name: &str, body: impl FnOnce()) {
    if std::env::var_os(CHILD_MARKER).is_some() {
        body();
        return;
    }

    // Drop the crate name: libtest paths start at the first module.
    let relative = module_path
        .split_once("::")
        .map_or(module_path, |(_, rest)| rest);
    let test_path = format!("{relative}::{name}");

    let exe = std::env::current_exe()
        .expect("the test executable path should be available");
    let output = Command::new(exe)
        .args([test_path.as_str(), "--exact", "--test-threads=1"])
        .env(CHILD_MARKER, "1")
        .output()
        .expect("the test executable should start");
    assert!(
        output.status.success(),
        "isolated test `{test_path}` failed:\n{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}
