<!-- TODO Reviewed ??-??-2026 -->

# Operating System

[![cat~os][cat~os~badge]][cat~os]

Rust applications can use portable operating-system services from the standard library and opt into platform-specific APIs when they need them. Start with a safe, portable interface; isolate platform-specific code and FFI behind a small boundary.

## Portable Operating-System Services

| Need                   | Starting point                                                                                  | Notes                                                                                                                 |
| ---------------------- | ----------------------------------------------------------------------------------------------- | --------------------------------------------------------------------------------------------------------------------- |
| Files and directories  | [`std::fs`][c~std::fs~docs]↗{{hi:std::fs}} and [`std::path`][c~std::path~docs]↗{{hi:std::path}} | Use path types instead of assuming a separator or filesystem layout.                                                  |
| Child processes        | [`std::process`][c~std::process~docs]↗{{hi:std::process}}                                       | Pass arguments as separate values and handle the exit status and I/O results.                                         |
| Environment variables  | [`std::env`][c~std::env~docs]↗{{hi:std::env}}                                                   | Treat values as external input and account for variables that may be absent.                                          |
| TCP and UDP networking | [`std::net`][c~std::net~docs]↗{{hi:std::net}}                                                   | Use an asynchronous runtime such as [`tokio`][c~tokio~docs]↗{{hi:tokio}} when the application needs asynchronous I/O. |
| Threads                | [`std::thread`][c~std::thread~docs]↗{{hi:std::thread}}                                          | Use message passing or synchronization primitives to share state safely.                                              |
| System information     | [`sysinfo`][c~sysinfo~docs]↗{{hi:sysinfo}}                                                      | Check which metrics are available on each supported operating system.                                                 |

## External Commands

Use a child process only when a library or standard API does not provide the needed behavior. Pass executable and arguments separately instead of building a shell command string. Check the exit code, capture or forward output intentionally, and account for platform-specific command availability.

{{#include external_commands.incl.md}}

## Process Management

Starting a child is easy; stopping it, together with the processes it spawned, is where programs go wrong. Use a process group (Unix) or a Job Object (Windows) to signal a whole process tree, wait for a child with a timeout, and guard a child so that it does not outlive its parent.

{{#include process_management.incl.md}}

## Signals and Graceful Shutdown

Catch Ctrl-C and `SIGTERM`, set a flag or send on a channel, and let the program flush its state and stop its workers.

{{#include signals.incl.md}}

## Platform-Specific APIs, System Calls, and FFI

The standard library exposes platform-specific extensions through modules such as [`std::os::unix`][c~std::os::unix~docs]↗{{hi:std::os::unix}} and [`std::os::windows`][c~std::os::windows~docs]↗{{hi:std::os::windows}}. Guard these imports and implementations with target configuration, then provide a portable alternative or a clear unsupported-platform error.

Direct system calls and Foreign Function Interface (FFI) bindings are lower-level choices. They require platform-specific types, calling conventions, resource cleanup, and error handling. Prefer a maintained safe wrapper when one covers the required API; otherwise keep the unsafe boundary small and document its invariants.

{{#include low_level_system_calls.incl.md}}

See the [Unix-specific APIs][p~unix] and [Linux APIs][p~linux] chapters for platform details.

## Operating Systems Written in Rust

This section lists operating systems and system projects implemented in Rust. They are useful for studying systems programming, but their presence here does not imply that application code should depend on them as a portable library.

{{#include rust_os.incl.md}}

## Related Topics {#related-topics .skip}

- [[filesystem | File System]] covers portable file and directory operations.
- [[command-line-interface | Command Line Interface]] covers command arguments and user interaction.
- [[network-programming | Network Programming]] covers networking beyond the standard library primitives.
- [[concurrency | Concurrency]] covers threads, synchronization, and parallel work.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  os/index: fix; organize; add cross links
</div>
