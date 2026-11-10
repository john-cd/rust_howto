//! Integration test that runs the profiling example binary as a separate process.
//!
//! TODO Reviewed ??-??-2026.

use std::process::Command;

/// The binary runs the dhat and time-measurement examples to completion.
#[test]
fn binary_runs_memory_and_timing_examples_successfully() {
    let output = Command::new(env!("CARGO_BIN_EXE_development_tools_profiling"))
        .output()
        .expect("the profiling binary should start");

    assert!(
        output.status.success(),
        "exit status {:?}; stderr: {}",
        output.status.code(),
        String::from_utf8_lossy(&output.stderr)
    );
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        combined.contains("execution_time"),
        "the timing example should report its measurement, got: {combined}"
    );
}
