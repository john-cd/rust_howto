<!-- TODO Reviewed ??-??-2026 -->

# IDEs

{{#include ides.incl.md}}

## Write Rust Code with VS Code {#vs-code}

[![cat~text-editors][cat~text-editors~badge]][cat~text-editors]

- **Setup:** Install Rust with `rustup`, then install the `rust-analyzer` extension (`rust-lang.rust-analyzer`) from the [Visual Studio Marketplace][rust-analyzer~plugin]↗ and open the folder containing the workspace `Cargo.toml`. The extension bundles the language server.
- **Debugging:** Install CodeLLDB on macOS or Linux, or CodeLLDB / Microsoft C++ on Windows. Start a session with the Rust Analyzer: Debug command or the Run/Debug CodeLens.
- **Tip:** Set `rust-analyzer.check.command` to `clippy` to see Clippy diagnostics while editing.

## Write Rust Code with RustRover {#rustrover}

[![cat~text-editors][cat~text-editors~badge]][cat~text-editors]

- **Setup:** Install [RustRover][rustrover~website]↗ and open a Cargo project. Check the detected Rust toolchain under Settings | Rust. RustRover provides its own Rust analysis engine; it does not use `rust-analyzer`.
- **Debugging:** Debug programs and tests with the built-in debugger. RustRover selects LLDB for the MSVC toolchain and GDB for the GNU toolchain on Windows; macOS uses LLDB, while Linux supports either debugger.
- **Tip:** Use the Cargo tool window or the automatically created project test configuration to run the whole workspace, then debug a failing test from its gutter action.

## Write Rust Code with the Helix Editor {#helix}

[![cat~text-editors][cat~text-editors~badge]][cat~text-editors]

- **Setup:** Install the Rust toolchain and the [`rust-analyzer`][rust-analyzer~website]↗ binary. Helix has a built-in Rust language-server configuration; use `hx --health` to check that the server is available. Install the standard-library sources with `rustup component add rust-src` if rust-analyzer cannot find them.
- **Debugging:** Helix includes a Debug Adapter Protocol (DAP) client. Install a compatible adapter such as CodeLLDB and configure its launch template for the Rust executable.
- **Tip:** Put project-specific language-server or debugger settings in `.helix/languages.toml` so they travel with the project.

## Write Rust Code with Zed {#zed}

[![cat~text-editors][cat~text-editors~badge]][cat~text-editors]

- **Setup:** Zed has native Rust support. Open a Cargo project; Zed uses `rust-analyzer` from `PATH` or installs its own stable copy.
- **Debugging:** Zed supports Rust debugging through CodeLLDB and GDB. Use the debugger panel or add a project configuration in `.zed/debug.json` for a custom Cargo command.
- **Tip:** Large workspaces can make check-on-save expensive. Set `rust-analyzer.check.workspace` to `false` to check the current package instead of the whole workspace.

## Write Rust Code with Neovim {#neovim}

[![cat~text-editors][cat~text-editors~badge]][cat~text-editors]

- **Setup:** Install Neovim 0.11 or later and the `rust-analyzer` binary. Enable the `rust_analyzer` LSP configuration with Neovim's built-in client, or install `nvim-lspconfig` for its maintained server configuration.
- **Debugging:** LSP does not provide a debugger. Add a DAP client such as `nvim-dap` and configure an adapter such as CodeLLDB separately.
- **Tip:** Run `:checkhealth vim.lsp` to confirm that the server is installed and attached to the Rust buffer.

## Write Rust Code with Emacs {#emacs}

[![cat~text-editors][cat~text-editors~badge]][cat~text-editors]

- **Setup:** Install the `rust-analyzer` binary and use Eglot (built into Emacs 29 and later) or `lsp-mode` as the LSP client. Both can start rust-analyzer for Rust buffers.
- **Debugging:** LSP provides editor intelligence, not a debugger. For a DAP workflow, configure `dap-mode` and a debugger adapter such as CodeLLDB separately.
- **Tip:** Eglot is the smaller built-in option. Choose `lsp-mode` if you need rust-analyzer-specific LSP extensions such as its extra commands.

## References {#references .skip}

- [RustRover installation guide][editor~rustrover~install]↗ and [debugging guide][editor~rustrover~debugging]↗.
- [Zed Rust support and debugging][editor~zed~rust]↗.
- [Helix language support][editor~helix~language-support]↗.
- [Neovim LSP documentation][editor~neovim~lsp]↗.
- [rust-analyzer's guide to Emacs and other editors][editor~rust-analyzer~other-editors]↗.

## Related Topics {#related-topics .skip}

- [[debugging | Debugging]].
- [[development_environment_setup | Development Environment Setup]].
- [[cargo | Cargo]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
</div>
