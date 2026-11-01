<!-- TODO Reviewed ??-??-2026 -->

# Arena Allocation

{{#include arena_allocation.incl.md}}

An _arena_{{hi:Arena}}{{hi:Bump allocation}} allocates many small objects that share one lifetime from a few large blocks. Each allocation only advances a pointer, and everything is freed at once when the arena is dropped. Consider an arena after profiling shows that allocation cost or fragmentation matters, for example when building a large tree or graph in one pass, and measure before and after. See [Memory Allocation][p~memory-allocation] for the basic `bumpalo` recipe.

## Build a Tree in a `bumpalo` Arena {#bumpalo-tree}

{{#badges bumpalo memory-management="Memory management" no-std="No standard library"}}{{hi:Arena}}{{hi:Tree}}

The nodes of this binary search tree live in a [`bumpalo`][c~bumpalo~docs]↗{{hi:bumpalo}} arena. They refer to each other with plain `&'bump` references: no `Box`, no `Rc`, one pointer bump per node. Dropping the arena frees the whole tree at once.

```rust,editable
{{#include ../../../crates/cats/memory_management/examples/bumpalo_tree.rs:example}}
```

Beware that `Drop` does not run for values allocated with `Bump::alloc`. Enable the `boxed` feature and use `bumpalo::boxed::Box` when destructors matter, for example for a value that owns a file handle.

## Related Topics {#related-topics .skip}

- [[memory_allocation | Custom Memory Allocation]].
- [[string_interning | String Interning]].
- [[lifetimes | Lifetimes]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [arena allocation: review the new page](https://github.com/john-cd/rust_howto/issues/1925)
</div>
