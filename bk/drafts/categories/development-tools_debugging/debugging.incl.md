<!-- TODO Reviewed ??-??-2026 -->

Use a debugger when you can reproduce a problem and need to pause execution, inspect variables, or step through control flow. Rust's `rust-gdb` and `rust-lldb` wrappers configure GDB and LLDB for Rust; VS Code users can follow the [Windows debugger setup note][p~debugging-in-vscode] for the debugger extensions and heap setting.

For a quick local inspection, [`dbg!`][debugger~dbg] prints the expression, its source location, and its value to standard error, then returns the value. Remove temporary calls before committing production code. For a panic's stack, set `RUST_BACKTRACE=1`; `std::backtrace::Backtrace::capture()` also uses `RUST_BACKTRACE` (or `RUST_LIB_BACKTRACE`) to decide whether to capture. Capturing can be expensive, so the environment variable lets you enable it when investigating a failure.

Logging and tracing are better when a problem needs to be observed repeatedly, in production, or across asynchronous work: logs record selected events, while tracing spans and events retain structured context. See [[log | Logging]] and [[tracing | Tracing]] for those approaches.

## Inspect an Expression with `dbg!` {#inspect-expression-with-dbg}

Use `dbg!` temporarily to see a value and the source location that produced it. The macro returns the expression unchanged, but it moves the value; borrow non-`Copy` values when you need to use them afterward.

{{#include ../../../crates/cats/development_tools_debugging/examples/debugging/dbg_macro.rs:example}}

## Capture a Backtrace {#capture-a-backtrace}

Set `RUST_BACKTRACE=1` before launching a program to capture a panic backtrace. Use `RUST_BACKTRACE=full` for more verbose panic output. In PowerShell, set it with `$env:RUST_BACKTRACE = "1"`; `Backtrace::capture()` is disabled unless `RUST_BACKTRACE` or `RUST_LIB_BACKTRACE` enables it.

{{#include ../../../crates/cats/development_tools_debugging/examples/debugging/backtrace.rs:example}}

| Recipe | Crates | Categories                                                                                                      |
| ------ | ------ | --------------------------------------------------------------------------------------------------------------- |
|        |        | [![cat~development-tools::debugging][cat~development-tools::debugging~badge]][cat~development-tools::debugging] |
