# Utilities

{{#include async_utilities.incl.md}}

## Visualize the Structure of Asynchronous Tasks and Their Dependencies {#utilities}

[![await-tree][c~await_tree~docs~badge]][c~await_tree~docs]{{hi:Await tree}}
[![await-tree~crates.io][c~await_tree~crates.io~badge]][c~await_tree~crates.io]
[![await-tree~github][c~await_tree~github~badge]][c~await_tree~github]
[![await-tree~lib.rs][c~await_tree~lib.rs~badge]][c~await_tree~lib.rs]
[![cat~asynchronous][cat~asynchronous~badge]][cat~asynchronous]{{hi:Asynchronous}}

The [`await-tree`][c~await_tree~docs]↗{{hi:await-tree}} crate provides a convenient way to visualize the structure of asynchronous tasks and their dependencies.

"The [Futures][p~futures] in [Async][p~async] Rust can be arbitrarily composited or nested to achieve a variety of control flows. Assuming that the execution of each Future is represented as a node, then the [asynchronous][p~async] execution of an [async][p~async] task can be organized into a logical tree, which is constantly transformed over the polling, completion, and cancellation of [Futures][p~futures]. [`await-tree`][c~await_tree~docs]↗{{hi:await-tree}} allows developers to dump this execution tree at runtime, with the span of each [`Future`][c~std::future::Future~docs]↗{{hi:Future}} annotated by `instrument_await`."

```rust,editable
{{#include ../../../crates/cats/concurrency/examples/await_tree.rs:example}}
```

## Related Topics {#related-topics}

- [[concurrency | Concurrency]].
- [[development_tools | Development Tools]].
- [[development-tools_profiling | Development Tools Profiling]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[review](https://github.com/john-cd/rust_howto/issues/906)
</div>
