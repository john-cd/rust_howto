// TODO Reviewed ??-??-2026.

// ANCHOR: example
//! Counts allocation calls made by the hand-written string interner.
//!
//! A string interner stores each distinct string once and hands out a small ID.
//! Comparing IDs is faster than comparing strings, and memory use falls when
//! the same text repeats, as in compilers and parsers.

use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;
use std::cell::Cell;

// This module is the same implementation used in the interning example.

/// Reuses the hand-written interner whose allocation behavior is measured here.
mod interning;

/// System allocator adapter recording successful allocation/reallocation calls per thread.
struct CountingAllocator;

thread_local! {
    // Counts per thread, so that other threads (such as a test harness) do
    // not disturb the measurement. A `const` initializer never allocates.

    /// Per-thread count initialized without allocating, isolating other test threads.
    static ALLOCATIONS: Cell<usize> = const { Cell::new(0) };
}

/// Reads this thread's cumulative count for before/after measurement snapshots.
fn allocations() -> usize {
    ALLOCATIONS.with(Cell::get)
}

/// Increments the current thread's count without allocating memory.
fn count_allocation() {
    ALLOCATIONS.with(|count| count.set(count.get() + 1));
}

// SAFETY: Each operation forwards the original pointer and layout to System;
// the counter uses a thread-local Cell and never allocates.
unsafe impl GlobalAlloc for CountingAllocator {
    /// Delegates allocation to System and counts only non-null results.
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The caller supplies the layout required by GlobalAlloc.
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            count_allocation();
        }
        ptr
    }

    /// Releases the allocation through System without changing the allocation-call count.
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: The caller supplies the original allocation and layout.
        unsafe { System.dealloc(ptr, layout) };
    }

    /// Delegates resizing and counts successful calls, even when the pointer stays unchanged.
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // SAFETY: The caller supplies the original allocation, layout, and new
        // size.
        let result = unsafe { System.realloc(ptr, layout, new_size) };
        if !result.is_null() {
            count_allocation();
        }
        result
    }
}

/// Installs the counting adapter for allocations throughout this example process.
#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

/// Measures one insertion followed by repeated lookups, before printing.
fn main() {
    run();
}

/// Measures allocations for one insertion and repeated interner lookups.
fn example() {
    interning::run();
    let mut interner = interning::Interner::default();
    let before = allocations();
    let symbol = interner.intern(std::hint::black_box("repeated identifier"));
    let after_insert = allocations();
    for _ in 0..1_000 {
        assert_eq!(
            interner.intern(std::hint::black_box("repeated identifier")),
            symbol
        );
    }
    let after_lookups = allocations();
    let insertion_calls = after_insert - before;
    let lookup_calls = after_lookups - after_insert;
    assert!(insertion_calls > 0);
    assert_eq!(lookup_calls, 0);
    println!(
        "First insertion: {insertion_calls} allocation calls; 1,000 repeat lookups: {lookup_calls}"
    );
}
// ANCHOR_END: example

/// Counts allocation calls made by the hand-written string interner.
fn run() {
    example();
}

/// Verifies that a first insertion allocates and repeated lookups do not.
#[cfg(test)]
mod tests {
    use super::*;

    /// Verifies that a first insertion allocates and repeated lookups do not.
    #[test]
    fn test_main_measures_allocations() {
        main();
        let before = allocations();
        drop(Box::new(0_u8));
        assert!(allocations() > before);
    }
}
