<!-- TODO Reviewed ??-??-2026 -->

# Signals and Graceful Shutdown

{{#include signals.incl.md}}

A program that is stopped with Ctrl-C (`SIGINT`) or `SIGTERM` is killed immediately by default, without flushing buffers, closing files or stopping worker threads. To shut down cleanly, install a handler, record that a shutdown was requested, and let the normal control flow of the program do the cleanup.

- Keep the handler tiny. Set an atomic flag or send on a channel, and do the real work in `main` or in the worker threads. Signal handlers run in an unusual context, and the standard library offers no guarantees about what is safe to do inside one at the operating-system level.
- Make the shutdown logic testable without sending a real signal: factor it into a function that takes a flag (or a future) and test it by setting the flag.
- On Windows, only Ctrl-C and Ctrl-Break are delivered; `SIGTERM` exists on Unix only.

## Shut Down Gracefully on Ctrl-C with `ctrlc` {#ctrlc}

{{#badges ctrlc os="Operating systems"}}

[`ctrlc::set_handler`][c~ctrlc~docs]↗{{hi:ctrlc::set_handler}} runs a closure on a dedicated thread when Ctrl-C is pressed. The example sets an `AtomicBool`, which a worker thread polls; `main` then joins the worker. Enable the crate's `termination` feature to also handle `SIGTERM` and `SIGHUP` on Unix, and the equivalent events on Windows.

```rust,editable,noplayground
{{#include ../../../crates/cats/os/examples/signals/ctrlc_example.rs:example}}
```

## Shut Down an Async Service on Ctrl-C or SIGTERM with `tokio::signal` {#tokio-signal}

{{#badges tokio os="Operating systems" asynchronous=Asynchronous}}

In async code, await [`tokio::signal::ctrl_c`][c~tokio::signal::ctrl_c~docs]↗{{hi:tokio::signal::ctrl_c}} in a `tokio::select!` next to the work to be stopped. On Unix, listen for `SIGTERM` as well with `tokio::signal::unix::signal(SignalKind::terminate())`. The service function takes the shutdown future as a parameter, so tests can pass a future they control.

```rust,editable,noplayground
{{#include ../../../crates/cats/os/examples/signals/tokio_signal.rs:example}}
```

## Related Topics {#related-topics .skip}

- [[process_management | Process Management]] covers stopping child processes.
- [[external_commands | External Commands]].
- [[concurrency | Concurrency]] covers threads and channels.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [graceful shutdown on Ctrl-C](https://github.com/john-cd/rust_howto/issues/1907)
</div>
