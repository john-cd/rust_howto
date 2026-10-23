<!-- TODO Reviewed ??-??-2026 -->
# Debuggers

Rust's toolchain provides `rust-gdb` and `rust-lldb` wrappers for launching GDB or LLDB with Rust type visualizers configured. Install the underlying debugger for your operating system, then build with Cargo's `dev` profile so the binary includes debug information.

For example, save this as `src/main.rs`:

```rust
fn main() {
    let answer = 40 + 2;
    println!("{answer}");
}
```

## Debug from a Terminal {#terminal-debuggers}

Build a binary and start GDB (use `app.exe` on Windows):

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

The variable name in `print answer` or `frame variable answer` must be in scope at the current frame. Debugger command names vary slightly between GDB and LLDB; use `help` at either prompt to inspect available commands.

## Debug with VS Code and CodeLLDB {#vscode}

Install [`rust-analyzer`][vscode~rust~docs]↗ and [CodeLLDB][debugger~codelldb]↗. VS Code's Rust guide recommends CodeLLDB on macOS and Linux; Windows users can use the Microsoft C++ debugger extension. After building with `cargo build`, add `.vscode/launch.json` with a launch configuration for the binary:

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

Change `program` to the binary Cargo built, including `.exe` on Windows. Set a breakpoint by clicking next to a source line, start the configuration from Run and Debug, and use the Variables and Call Stack panes to inspect the stopped program.

## Panic Backtraces and Debug Symbols {#backtraces-and-symbols}

For a quick inspection while editing, [`dbg!`][debugger~dbg]↗ prints an expression and its source location to standard error. For a panic backtrace, set `RUST_BACKTRACE=1` before running the program:

```bash
RUST_BACKTRACE=1 cargo run
```

In PowerShell, use `$env:RUST_BACKTRACE = "1"` before `cargo run`. Cargo's `dev` profile includes full debug information by default. To retain reduced debug information in optimized builds, configure the workspace root manifest:

```toml
[profile.release]
debug = 1
```

See the [Cargo profile reference][debugger~cargo~profiles]↗ for other debug information settings. For application diagnostics, see [[log | Logging]] and [[tracing | Tracing]].

## Related Topics {#related-topics .skip}

- [[debugging | Debugging]].
- [[development-tools_testing | Testing]].
- [[development-tools_profiling | Profiling]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[add Debugger page](https://github.com/john-cd/rust_howto/issues/50)
</div>
