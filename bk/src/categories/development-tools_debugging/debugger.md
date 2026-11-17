<!-- TODO Reviewed ??-??-2026. -->

# Debuggers

Rust's toolchain provides `rust-gdb` and `rust-lldb` wrappers that launch GDB or LLDB with Rust type visualizers configured. Install the debugger for your operating system, then build with Cargo's `dev` profile, which includes debug information by default.

This small program has a local value to inspect while stepping through execution:

```rust
{{#include ../../../crates/cats/development_tools_debugging/examples/debugger/debugger_walkthrough.rs:example}}
```

{{#include debugger.incl.md}}

## Debug from a Terminal {#terminal-debuggers}

Build the binary, then start GDB with the binary's actual name (use its `.exe` path on Windows):

```console
cargo build
rust-gdb target/debug/app
```

At the debugger prompt, set a breakpoint, run the program, step to the next line, inspect a local variable, and print the stack:

```text
(gdb) break main
(gdb) run
(gdb) next
(gdb) print answer
(gdb) backtrace
```

With LLDB, use the corresponding commands:

```console
rust-lldb target/debug/app
```

```text
(lldb) breakpoint set --name main
(lldb) run
(lldb) thread step-over
(lldb) frame variable answer
(lldb) bt
```

The variable in `print answer` or `frame variable answer` must be in scope at the selected frame. GDB and LLDB command names vary; use `help` at either prompt to inspect available commands.

## Debug with VS Code and CodeLLDB {#vscode}

Install the [`rust-analyzer`][vscode~rust~docs]↗ extension for Rust language support and the [CodeLLDB][debugger~codelldb]↗ extension for debugging. After building with `cargo build`, add `.vscode/launch.json` with a launch configuration for the binary:

```json
{
  "version": "0.2.0",
  "configurations": [
    {
      "type": "lldb",
      "request": "launch",
      "name": "Debug Rust binary",
      "program": "${workspaceFolder}/target/debug/app",
      "args": [],
      "cwd": "${workspaceFolder}"
    }
  ]
}
```

Change `program` to the path Cargo built, including `.exe` on Windows. Set a breakpoint beside a source line, start the configuration from Run and Debug, then inspect the Variables and Call Stack panes.

## Panic Backtraces and Debug Symbols {#backtraces-and-symbols}

For a panic backtrace, set `RUST_BACKTRACE=1` before running the program:

```bash
RUST_BACKTRACE=1 cargo run
```

In PowerShell, set `$env:RUST_BACKTRACE = "1"` before `cargo run`. `std::backtrace::Backtrace::capture()` also honors `RUST_BACKTRACE` or `RUST_LIB_BACKTRACE`. To retain reduced debug information in optimized builds, configure the workspace root manifest:

```toml
[profile.release]
debug = 1
```

See the [Cargo profile reference][debugger~cargo~profiles]↗ for other debug information settings. For runtime application diagnostics, see [Log Messages](log.md) and [Tracing](tracing.md).

## Related Topics {#related-topics .skip}

- [Debugging](debugging.md) for `dbg!`, backtraces, and source locations.
- [Development Tools - Testing](../development-tools_testing/testing.md).
- [Development Tools - Profiling](../development-tools_profiling/index.md).

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
