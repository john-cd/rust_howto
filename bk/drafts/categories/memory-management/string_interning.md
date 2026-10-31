<!-- TODO Reviewed ??-??-2026 -->

# String Interning

{{#include string_interning.incl.md}}

_Interning_{{hi:Interning}}{{hi:String interning}} stores each distinct string once and hands out cheap handles. Comparing, hashing and copying a handle costs the same regardless of the string's length, which helps a program that compares the same identifiers many times, such as a compiler, a parser or a code-analysis tool. The price is that the pool keeps every string alive. Measure the effect on your workload before adopting it.

## Intern Strings by Hand or with `ustr` {#interning}

{{#badges ustr memory-management="Memory management" data-structures="Data structures"}}{{hi:Interning}}{{hi:String interning}}{{hi:Symbol table}}

The example shows two interners:

- A hand-written one: a `HashMap<String, u32>` maps each string to an id, and a `Vec<String>` maps the id back to the string. Ids are small, `Copy`, and compare as integers.
- The [`ustr`][c~ustr~docs]↗{{hi:ustr}} crate. A `Ustr` is a pointer into a global string pool, so equality and hashing are pointer operations. The pool is never freed: the memory of an interned string is not reclaimed until the process exits.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/interning.rs:example}}
```

## Count Allocations for Repeated Interning

The separate `interner_allocations` executable wraps the system allocator and counts successful allocation and reallocation calls. Run it with `just cargo run --manifest-path crates/Cargo.toml -p memory_management --example interner_allocations` from `bk/`. It measures the hand-written interner above: a first insertion allocates storage, while repeated lookups of the same borrowed string require no additional allocations. The counter is process-wide, so the example takes its measurements on the main thread without printing inside the measured intervals. Counts are allocator calls, not bytes, peak memory, or a performance benchmark; measure representative workloads before drawing broader conclusions.

```rust,editable,noplayground
{{#include ../../../crates/cats/memory_management/examples/interner_allocations.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[arena_allocation | Arena Allocation]].
- [[memory_allocation | Custom Memory Allocation]].
- [[strings | Strings]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [string interning: review the new page](https://github.com/john-cd/rust_howto/issues/1925)
</div>
