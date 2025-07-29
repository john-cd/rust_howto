# Text Editors

[![cat~text-editors][cat~text-editors~badge]][cat~text-editors]{{hi:Text editors}}

Applications for editing text.

For the most comprehensive and user-friendly experience, VS Code (with the `rust-analyzer` extension), IntelliJ IDEA (with the Rust plugin) or CLion (with the Rust plugin) are highly recommended. VS Code, in particular, has become popular due to its performance and the quality of the `rust-analyzer` extension.

Vim, Emacs, and Neovim offer great flexibility and power, but they require a higher initial configuration investment. Sublime Text and Atom can also be configured for decent Rust support, but VS Code has largely taken over in popularity.

## IDEs and Text Editors with Rust Support

| IDE/Editor | Rust Support Level | Key Features/Notes |
|---|---|---|
| [VS Code][rust-in-vs-code]{{hi:VS code}}⮳ (with `rust-analyzer` extension) | Excellent | [`rust-analyzer`][rust-analyzer~website]⮳{{hi:rust-analyzer}} provides excellent language server support, offering code completion, diagnostics, refactoring, and more. Also consider the "CodeLLDB", "Dependi" and "Even Better TOML" extensions. |
| [RustRover][rustrover~website]⮳ | Excellent | It is available for free for non-commercial use. |
| IntelliJ IDEA (with Rust plugin) | Excellent | Comprehensive support including code completion, refactoring, debugging, navigation, and integration with Cargo. The official Rust plugin provides the most complete IDE experience. |
| CLion JetBrains IDE (with Rust plugin) | Excellent | CLion, JetBrains' C/C++ IDE, also has a Rust plugin that leverages `rust-analyzer`. Provides a robust IDE experience similar to IntelliJ IDEA.  If you have a JetBrains license, CLion is your go-to editor for Rust in JetBrains' IDE suite. |
| [Helix][helix-editor~website]⮳[(github)][helix-editor~github]⮳{{hi:helix}} | | Install the [rust-analyzer binary][rust-analyzer~helix~website]⮳ |
| [Zed][c~zed~website]⮳ | Excellent | `zed` is available for macOS, Linux, and soon for Windows. Written in Rust. |
| [Sublime Text][sublime-text~website]⮳ (with LSP and related plugins) | Good | Can achieve good Rust support via the LSP (Language Server Protocol) and plugins like `rust-analyzer`. Requires some setup. See ["Rust enhanced" package][sublime-text-rust-enhanced~github]⮳. |
| [Atom][atom~website]⮳ (with LSP and related packages) | Good | Similar to `Sublime Text`, Atom can be configured with LSP and `rust-analyzer` for decent Rust support. `Atom` is no longer actively developed. |
| Vim (with plugins like `coc.nvim` or `vim-lsp`) | Good (Highly Configurable) | With the right plugins (e.g., `coc.nvim`, `vim-lsp`, and `rust-analyzer`), Vim can be a very powerful Rust editor. Requires more configuration than other options. Configure [Vim for Rust][rust-vim~github]⮳. |
| [Emacs][emacs~website]⮳ with plugins like `lsp-mode` or `eglot` (or derivatives like Doom Emacs, [Spacemacs][spacemacs~website]⮳, etc.) | Good (Highly Configurable) | Similar to Vim, Emacs can be configured for excellent Rust development with LSP and `rust-analyzer`. Also requires a learning curve for configuration. Configure [Emacs for Rust][emacs-rust-mode~github]⮳. |
| [Neovim][nvim~website]⮳ (with plugins like `coc.nvim` or `nvim-lsp`) | Good (Highly Configurable) | Neovim, a fork of `vim`, also benefits from the LSP and `rust-analyzer` through plugins like `coc.nvim` or `nvim-lsp`. |
| Kate | Decent | Provides syntax highlighting and some basic code completion. LSP support might be available through plugins. |
| [Visual Studio][visual-studio~website]⮳ | | Configure [rust-analyzer for Visual Studio][rust-analyzer~visual-studio~website]⮳ |
| [Lapce][lapce~website]⮳ | | Open source, written in Rust |
| [Xcode][xcode~website]⮳ | | |
| [Eclipse Corrosion][eclipse-corrosion~website]⮳ | | It provides development tools for Rust and Cargo inside the Eclipse IDE |

## Examples

{{#include ides.incl.md}}

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[write; review in depth finish the table NOW](https://github.com/john-cd/rust_howto/issues/962)
</div>
