# Tokio

[![tokio][c-tokio-badge]][c-tokio]{{hi:tokio}} [![tokio-crates.io][c-tokio-crates.io-badge]][c-tokio-crates.io]
[![tokio-github][c-tokio-github-badge]][c-tokio-github] [![tokio-lib.rs][c-tokio-lib.rs-badge]][c-tokio-lib.rs] [![cat-asynchronous][cat-asynchronous-badge]][cat-asynchronous]{{hi:Asynchronous}} [![cat-network-programming][cat-network-programming-badge]][cat-network-programming]{{hi:Network programming}}

## Basics

- creating and running a runtime{{hi:Runtime}}, spawning tasks{{hi:Spawning tasks}}, working with I/O and timers, and handling errors.

### Join

By running all async{{hi:async}} expressions on the current task, the expressions are able to run concurrently but not in parallel. This means all expressions are run on the same thread and if one branch blocks the thread, all other expressions will be unable to continue. If parallelism is required, spawn each async expression using `tokio::spawn`{{hi:tokio::spawn}} and pass the join handle to `join!`{{hi:join!}}.

### Spawning

## IO

- The I/O section of the website explains how to read and write data asynchronously with Tokio, using streams, codecs, and futures. It also shows how to handle errors and timeouts.

[Current thread runtime][c-tokio::main::current-thread-runtime]{{hi:tokio::main::current-thread-runtime}}⮳

equivalent to

```rust
{{#include ../../../deps/tests/cats/asynchronous/tokio2.rs:example}}
```

[LocalSet][c-tokio::task::LocalSet]{{hi:tokio::task::LocalSet}}⮳

In some cases, it is necessary to run one or more futures that do not implement Send{{hi:Send}} and thus are unsafe to send between threads. In these cases, a local task set may be used to schedule one or more `!Send` futures to run together on the same thread.

```rust
{{#include ../../../deps/tests/cats/asynchronous/tokio22.rs:example}}
```

{{#include ../../refs/link-refs.md}}

<div class="hidden">
TODO: merge with tokio.md
</div>
