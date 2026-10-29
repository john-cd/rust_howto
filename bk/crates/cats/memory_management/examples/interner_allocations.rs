//! Counts allocation calls made by the hand-written string interner.

// ANCHOR: example
use std::alloc::GlobalAlloc;
use std::alloc::Layout;
use std::alloc::System;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering;

// This module is the same implementation used in the interning example.
mod interning;

struct CountingAllocator;
static ALLOCATIONS: AtomicUsize = AtomicUsize::new(0);

// SAFETY: Each operation forwards the original pointer and layout to System;
// the counter uses only atomics and never allocates.
unsafe impl GlobalAlloc for CountingAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // SAFETY: The caller supplies the layout required by GlobalAlloc.
        let ptr = unsafe { System.alloc(layout) };
        if !ptr.is_null() {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        ptr
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        // SAFETY: The caller supplies the original allocation and layout.
        unsafe { System.dealloc(ptr, layout) };
    }

    unsafe fn realloc(
        &self,
        ptr: *mut u8,
        layout: Layout,
        new_size: usize,
    ) -> *mut u8 {
        // SAFETY: The caller supplies the original allocation, layout, and new
        // size.
        let result = unsafe { System.realloc(ptr, layout, new_size) };
        if !result.is_null() {
            ALLOCATIONS.fetch_add(1, Ordering::Relaxed);
        }
        result
    }
}

#[global_allocator]
static GLOBAL: CountingAllocator = CountingAllocator;

/// Measures one insertion followed by repeated lookups, before printing.
fn main() {
    interning::run();
    let mut interner = interning::Interner::default();
    let before = ALLOCATIONS.load(Ordering::Relaxed);
    let symbol = interner.intern(std::hint::black_box("repeated identifier"));
    let after_insert = ALLOCATIONS.load(Ordering::Relaxed);
    for _ in 0..1_000 {
        assert_eq!(
            interner.intern(std::hint::black_box("repeated identifier")),
            symbol
        );
    }
    let after_lookups = ALLOCATIONS.load(Ordering::Relaxed);
    let insertion_calls = after_insert - before;
    let lookup_calls = after_lookups - after_insert;
    assert!(insertion_calls > 0);
    assert_eq!(lookup_calls, 0);
    println!(
        "First insertion: {insertion_calls} allocation calls; 1,000 repeat lookups: {lookup_calls}"
    );
}
// ANCHOR_END: example
