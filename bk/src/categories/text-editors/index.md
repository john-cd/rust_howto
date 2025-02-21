# Text Editors

[![cat-text-editors][cat-text-editors-badge]][cat-text-editors]{{hi:Text editors}}

Applications for editing text.

{{#include ides.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[P1 review](https://github.com/john-cd/rust_howto/issues/962)

## IDEs and Text Editors with Rust Support

This table lists popular IDEs and text editors that offer varying levels of Rust support, from basic syntax highlighting to advanced language features.

| IDE/Editor | Rust Support Level | Key Features/Notes |
|---|---|---|
| IntelliJ IDEA (with Rust plugin) | Excellent | Comprehensive support including code completion, refactoring, debugging, navigation, and integration with Cargo. The official Rust plugin provides the most complete IDE experience. |
| VS Code (with rust-analyzer extension) | Excellent | Highly recommended. `rust-analyzer` provides excellent language server support, offering code completion, diagnostics, refactoring, and more. A very popular and powerful setup. |
| CLion (with Rust plugin) | Excellent | CLion, JetBrains' C/C++ IDE, also has a Rust plugin that leverages `rust-analyzer`. Provides a robust IDE experience similar to IntelliJ IDEA. |
| Sublime Text (with LSP and related plugins) | Good | Can achieve good Rust support via the LSP (Language Server Protocol) and plugins like `rust-analyzer`. Requires some setup. |
| Atom (with LSP and related packages) | Good | Similar to Sublime Text, Atom can be configured with LSP and `rust-analyzer` for decent Rust support. Atom is no longer actively developed. |
| Vim (with plugins like coc.nvim or vim-lsp) | Good (Highly Configurable) | With the right plugins (e.g., `coc.nvim`, `vim-lsp`, and `rust-analyzer`), Vim can be a very powerful Rust editor. Requires more configuration than other options. |
| Emacs (with plugins like lsp-mode or eglot) | Good (Highly Configurable) | Similar to Vim, Emacs can be configured for excellent Rust development with LSP and `rust-analyzer`. Also requires a learning curve for configuration. |
| Neovim (with plugins like coc.nvim or nvim-lsp) | Good (Highly Configurable) | Neovim, a fork of Vim, also benefits from the LSP and `rust-analyzer` through plugins like `coc.nvim` or `nvim-lsp`. |
| Kate | Decent | Provides syntax highlighting and some basic code completion. LSP support might be available through plugins. |

## Recommendations

For the most comprehensive and user-friendly experience, IntelliJ IDEA (with the Rust plugin), VS Code (with the `rust-analyzer` extension), or CLion (with the Rust plugin) are highly recommended. VS Code, in particular, has become incredibly popular due to its excellent performance and the quality of the `rust-analyzer` extension.

Vim, Emacs, and Neovim offer great flexibility and power, but they require a higher initial configuration investment. Sublime Text and Atom can also be configured for decent Rust support, but VS Code has largely taken over in popularity.

</div>
