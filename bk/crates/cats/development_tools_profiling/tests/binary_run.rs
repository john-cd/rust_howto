//! Integration test that runs the profiling example binary as a separate process.
//!
//! TODO Reviewed ??-??-2026.

use std::process::Command;

/// The binary runs the dhat and time-measurement examples to completion.
#[test]
fn binary_runs_memory_and_timing_examples_successfully() {
    let binary = env!("CARGO_BIN_EXE_development_tools_profiling");
    let memory = Command::new(binary)
        .arg("dhat")
        .output()
        .expect("the profiling binary should start");

    assert!(
        memory.status.success(),
        "exit status {:?}; stderr: {}",
        memory.status.code(),
        String::from_utf8_lossy(&memory.stderr)
    );

    let timing = Command::new(binary)
        .arg("measure-time")
        .output()
        .expect("the profiling binary should start");
    let combined = format!(
        "{}{}",
        String::from_utf8_lossy(&timing.stdout),
        String::from_utf8_lossy(&timing.stderr)
    );
    assert!(
        timing.status.success(),
        "exit status {:?}; stderr: {combined}",
        timing.status.code()
    );
    assert!(
        combined.contains("execution_time"),
        "the timing example should report its measurement, got: {combined}"
    );
}
