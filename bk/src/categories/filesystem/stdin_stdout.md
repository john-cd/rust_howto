# Working with the Standard Input and Output

{{#include stdin_stdout.incl.md}}

## Read from the Standard Input and Write to the Standard Output or Standard Error {#stdin-stdout}

`std::io::stdin`, `stdout`, and `stderr` are handles to the standard input, standard output, and standard error streams of a process, respectively. They are your primary tools for interacting with a user via the command line. `stdin`, by default, is connected to the keyboard; `stdout` and `stderr` are connected to the terminal or console window.

The following demonstrates how to work with them. You will rarely need to explicitly call `stdout` or `stderr` unless you need to lock them (to prevent output interleaving):

```rust,editable
{{#include ../../../crates/cats/filesystem/examples/stdin_stdout/stdin_stdout.rs:example}}
```

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
