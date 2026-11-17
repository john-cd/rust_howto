<!-- TODO Reviewed ??-??-2026. -->

Use a debugger when you can reproduce a problem and need to pause execution, inspect variables, or step through control flow. Rust's `rust-gdb` and `rust-lldb` wrappers configure GDB and LLDB for Rust; VS Code users can use the CodeLLDB extension. See [Debuggers](debugger.md) for setup and command examples.

For a quick local inspection, [`dbg!`][debugger~dbg] prints an expression, its source location, and its value to standard error, then returns the expression. The macro moves the value; borrow non-`Copy` values when you need to use them afterward. Remove temporary calls before committing production code.

For panic backtraces, set `RUST_BACKTRACE=1` before launching the program. `std::backtrace::Backtrace::capture()` also uses `RUST_BACKTRACE` or `RUST_LIB_BACKTRACE` to decide whether to capture a backtrace. Capturing can be expensive, so these environment variables let you enable it while investigating a failure.

Logging and tracing help when a problem needs to be observed repeatedly, in production, or across asynchronous work: logs record selected events, while tracing spans and events retain structured context. See [Log Messages](log.md) and [Tracing](tracing.md).

## Inspect an Expression with `dbg!` {#inspect-expression-with-dbg}

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/debugging/dbg_macro.rs:example}}
```

## Capture a Backtrace {#capture-a-backtrace}

Set `RUST_BACKTRACE=1` before launching a program to enable panic backtraces. Use `RUST_BACKTRACE=full` for more verbose panic output. In PowerShell, set it with `$env:RUST_BACKTRACE = "1"`.

```rust,editable
{{#include ../../../crates/cats/development_tools_debugging/examples/debugging/backtrace.rs:example}}
```

| Recipe | Crates | Categories |
| --- | --- | --- |
| [Inspect an Expression with `dbg!`][ex~development-tools_debugging~inspect-expression-with-dbg] | `std` | {{#categories development-tools::debugging=Debugging}} |
| [Capture a Backtrace][ex~development-tools_debugging~capture-a-backtrace] | `std` | {{#categories development-tools::debugging=Debugging}} |
