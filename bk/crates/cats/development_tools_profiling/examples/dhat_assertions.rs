// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Asserts on heap allocations in a test with `dhat`.

#[global_allocator]
static ALLOC: dhat::Alloc = dhat::Alloc;

/// Builds a string without growing it: one allocation.
fn with_capacity(parts: &[&str]) -> String {
    let mut s = String::with_capacity(parts.iter().map(|p| p.len()).sum());
    for part in parts {
        s.push_str(part);
    }
    s
}

/// Runs the allocation example outside the test harness.
fn main() {
    run();
}

pub fn run() {
    println!("{}", with_capacity(&["a", "b", "c"]));
}

#[cfg(test)]
mod tests {
    use super::*;

    // `dhat` allows one profiler at a time per process, so keep all the
    // allocation assertions in a single test.
    /// Verifies the optimized string builder performs one allocation.
    #[test]
    fn allocation_counts() {
        let _profiler = dhat::Profiler::builder().testing().build();
        let parts = ["alpha", "beta", "gamma"];

        let before = dhat::HeapStats::get();
        let s = with_capacity(&parts);
        let after = dhat::HeapStats::get();

        assert_eq!(s, "alphabetagamma");
        // Exactly one block was allocated, and it was not reallocated.
        dhat::assert_eq!(after.total_blocks - before.total_blocks, 1);
        dhat::assert!(after.total_bytes - before.total_bytes >= 14);
        // The final block is still alive; nothing else leaked.
        dhat::assert_eq!(after.curr_blocks - before.curr_blocks, 1);
    }

    #[test]
    fn test_main_entry_point() {
        main();
    }
}
// ANCHOR_END: example
