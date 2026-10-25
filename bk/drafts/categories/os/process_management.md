<!-- TODO Reviewed ??-??-2026 -->

# Process Management

{{#include process_management.incl.md}}

[`std::process::Command`][c~std::process::Command~docs]↗ makes starting a child process easy (see [[external_commands | External Commands]]). Cleaning up reliably is the part that is easy to get wrong:

- Killing a child does not kill the processes it started. Put the child in its own process group (Unix) or a Job Object (Windows) and signal the whole group.
- Always `wait` for a child after killing it; otherwise it remains a zombie until the parent exits.
- A child that outlives a parent that returned early, or panicked, keeps running. Guard it with a type that kills it in `Drop`.
- A child that hangs blocks its parent forever unless the parent waits with a timeout.

Do not use these techniques to isolate untrusted code on their own: a process group or a Job Object only ties lifetimes together.

## Kill a Process Tree with a Unix Process Group {#kill-a-process-group}

[![nix][c~nix~docs~badge]][c~nix~docs]{{hi:nix}} [![nix~crates.io][c~nix~crates.io~badge]][c~nix~crates.io] [![nix~repo][c~nix~repo~badge]][c~nix~repo] [![nix~lib.rs][c~nix~lib.rs~badge]][c~nix~lib.rs] [![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}

`CommandExt::process_group(0)` makes the child the leader of a new group whose id equals the child's pid. [`nix::sys::signal::killpg`][c~nix::sys::signal::killpg~docs]↗{{hi:killpg}} then signals every member of the group, including grandchildren. Send `SIGTERM` first if the processes should get a chance to clean up, and `SIGKILL` as a last resort. The test spawns a shell that starts a background `sleep`, and checks that the `sleep` is gone after the group is killed.

```rust,editable,noplayground
{{#include ../../../crates/cats/os/examples/process_management/process_group.rs:example}}
```

## Tie a Windows Child's Lifetime to a Job Object {#job-object}

[![windows-sys][c~windows-sys~docs~badge]][c~windows-sys~docs]{{hi:windows-sys}} [![windows-sys~crates.io][c~windows-sys~crates.io~badge]][c~windows-sys~crates.io] [![windows-sys~repo][c~windows-sys~repo~badge]][c~windows-sys~repo] [![windows-sys~lib.rs][c~windows-sys~lib.rs~badge]][c~windows-sys~lib.rs] [![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}

A Windows Job Object configured with `JOB_OBJECT_LIMIT_KILL_ON_JOB_CLOSE` terminates every process assigned to it when the last handle to the job is closed. The example wraps the job handle in a type that implements `Drop`, so the tree dies with the parent. It is compiled only on Windows (`#[cfg(windows)]`), and is shown here as a snippet.

```rust,ignore
{{#include ../../../crates/cats/os/examples/process_management/job_object.rs:example}}
```

## Run a Command with a Timeout, then Kill it {#timeout-kill}

[![wait-timeout][c~wait-timeout~docs~badge]][c~wait-timeout~docs]{{hi:wait-timeout}} [![wait-timeout~crates.io][c~wait-timeout~crates.io~badge]][c~wait-timeout~crates.io] [![wait-timeout~repo][c~wait-timeout~repo~badge]][c~wait-timeout~repo] [![wait-timeout~lib.rs][c~wait-timeout~lib.rs~badge]][c~wait-timeout~lib.rs] [![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}

`std::process::Child` has no wait with a timeout. The [`wait-timeout`][c~wait-timeout~docs]↗ crate adds `ChildExt::wait_timeout`. If it returns `None`, kill the child and `wait` for it to reap it. With `tokio`, use `tokio::time::timeout` around `Child::wait`, or `Child::kill` in an async task.

```rust,editable,noplayground
{{#include ../../../crates/cats/os/examples/process_management/timeout_kill.rs:example}}
```

## Kill a Child Automatically with a `Drop` Guard {#kill-guard}

[![std][c~std~docs~badge]][c~std~docs]{{hi:std}} [![cat~os][cat~os~badge]][cat~os]{{hi:Operating systems}}

Wrap the child in a type that implements [`Drop`][c~std::ops::Drop~docs]↗{{hi:Drop}} and kills and reaps it. The child then does not outlive the parent when the parent returns early with `?` or unwinds after a panic. Provide a method to release the child when it should keep running.

```rust,editable,noplayground
{{#include ../../../crates/cats/os/examples/process_management/kill_guard.rs:example}}
```

## Confine Untrusted Programs {#confine-untrusted-programs .skip}

Controlling the lifetime of a child is different from limiting what it can do. To confine an untrusted program:

- On Linux, lower resource limits with `setrlimit` or the `prlimit` command, and run the program in new namespaces, for example with `bwrap` (Bubblewrap). The Landlock security module restricts file system access from inside the process. Check that the tools are available before relying on them, and fail closed when they are not.
- On Windows, use Job Objects to limit memory, CPU time and the number of processes, and AppContainer to restrict access to resources.
- To isolate untrusted code in the same process, run it as WebAssembly in a runtime like Wasmtime instead (see [#1926](https://github.com/john-cd/rust_howto/issues/1926)).

## Related Topics {#related-topics .skip}

- [[external_commands | External Commands]].
- [[signals | Signals and Graceful Shutdown]].
- [[concurrency | Concurrency]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [manage child processes](https://github.com/john-cd/rust_howto/issues/1922)
</div>
