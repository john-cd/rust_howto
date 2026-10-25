<!-- TODO Reviewed ??-??-2026 -->

# String Interning

{{#include string_interning.incl.md}}

*Interning*{{hi:Interning}}{{hi:String interning}} stores each distinct string once and hands out cheap handles. Comparing, hashing and copying a handle costs the same regardless of the string's length, which helps a program that compares the same identifiers many times, such as a compiler, a parser or a code-analysis tool. The price is that the pool keeps every string alive. Measure the effect on your workload before adopting it.

## Intern Strings by Hand or with `ustr` {#interning}

[![ustr][c~ustr~docs~badge]][c~ustr~docs] [![ustr~crates.io][c~ustr~crates.io~badge]][c~ustr~crates.io] [![ustr~repo][c~ustr~repo~badge]][c~ustr~repo] [![ustr~lib.rs][c~ustr~lib.rs~badge]][c~ustr~lib.rs]{{hi:ustr}}{{hi:Interning}}{{hi:String interning}}{{hi:Symbol table}} [![cat~memory-management][cat~memory-management~badge]][cat~memory-management]{{hi:Memory management}} [![cat~data-structures][cat~data-structures~badge]][cat~data-structures]{{hi:Data structures}}

The example shows two interners:

- A hand-written one: a `HashMap<String, u32>` maps each string to an id, and a `Vec<String>` maps the id back to the string. Ids are small, `Copy`, and compare as integers.
- The [`ustr`][c~ustr~docs]↗{{hi:ustr}} crate. A `Ustr` is a pointer into a global string pool, so equality and hashing are pointer operations. The pool is never freed: the memory of an interned string is not reclaimed until the process exits.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/interning.rs:example}}
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
