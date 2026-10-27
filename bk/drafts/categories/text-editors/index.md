<!-- TODO Reviewed ??-??-2026 -->

# Text Editors

[![cat~text-editors][cat~text-editors~badge]][cat~text-editors]{{hi:Text editors}}

These editors provide Rust support either directly or through a Rust language server such as [`rust-analyzer`][rust-analyzer~website]↗{{hi:rust-analyzer}}. Setup and available features vary by editor and plugins; the notes below describe the main integration path rather than rank editors.

## IDEs and Text Editors with Rust Support

| IDE/Editor                                                               | Rust integration                | Setup notes                                                                                                                              |
| ------------------------------------------------------------------------ | ------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------- |
| [VS Code][rust-in-vs-code~website]↗{{hi:VS code}}                        | `rust-analyzer` extension       | Provides completion, diagnostics, navigation, and refactoring. Extensions such as CodeLLDB add debugging support.                        |
| [RustRover][rustrover~website]↗                                          | Built-in Rust IDE support       | JetBrains IDE focused on Rust, with Cargo integration, code insight, and debugging.                                                      |
| [IntelliJ IDEA][intellij-idea~website]↗                                  | Rust plugin                     | Install JetBrains' Rust plugin to add Rust language and Cargo support.                                                                   |
| [CLion][clion~website]↗                                                  | Rust plugin                     | Install JetBrains' Rust plugin; CLion also provides its C/C++ tooling.                                                                   |
| [Helix][helix-editor~website]↗ ([GitHub][helix-editor~repo]){{hi:helix}} | Built-in language-server client | Configure the [rust-analyzer binary][rust-analyzer~helix~website]↗ for Rust diagnostics and code actions.                                |
| [Zed][c~zed~website]↗                                                    | Rust extension                  | Uses `rust-analyzer` for Rust language features.                                                                                         |
| [Sublime Text][sublime-text~website]↗                                    | LSP package                     | Install the LSP package and `rust-analyzer`; see the [Rust Enhanced package][sublime-text-rust-enhanced~repo]↗ for additional tooling.   |
| [Vim][vim~website]↗                                                      | Language-server plugin          | Configure a client such as [`coc.nvim`][coc.nvim~repo]↗{{hi:coc.nvim}} or [`vim-lsp`][vim-lsp~repo]↗{{hi:vim-lsp}} with `rust-analyzer`. |
| [Neovim][nvim~website]↗                                                  | Built-in LSP client             | Configure `rust-analyzer` directly or through a plugin such as `coc.nvim`.                                                               |
| [Emacs][emacs~website]↗                                                  | LSP package                     | Configure `rust-analyzer` with [`eglot`][eglot~repo]↗{{hi:eglot}} or [`lsp-mode`][emacs-lsp~website]↗{{hi:lsp-mode}}.                    |
| [Kate][kate-editor~website]↗                                             | LSP client plugin               | Install the LSP Client plugin and configure `rust-analyzer`; Kate also provides syntax highlighting.                                     |
| [Visual Studio][visual-studio~website]↗                                  | Rust Analyzer extension         | Install and configure the [Rust Analyzer extension][rust-analyzer~visual-studio~website]↗. Visual Studio is separate from VS Code.       |
| [Lapce][lapce~website]↗                                                  | Built-in LSP client             | Configure `rust-analyzer` for completion and diagnostics.                                                                                |
| [Eclipse Corrosion][eclipse-corrosion~website]↗                          | Corrosion plugin                | Integrates `rust-analyzer`, Cargo, and debugger support into Eclipse.                                                                    |

GitHub [sunset Atom on December 15, 2022](https://github.blog/2022-06-08-sunsetting-atom/). Atom may still edit Rust source, but it no longer receives project maintenance or updates, so it is not listed as a current Rust editor.

## Examples

{{#include ides.incl.md}}

## Related Topics

- [Development Tools][p~development-tools]
- [Text processing](../text-processing/index.md)

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">

The table distinguishes native support from plugin and LSP setups, fills in integration details, removes Atom from the current-editor list because GitHub sunset it, and replaces the unfinished related-topics placeholder. Recheck vendor setup links when updating this index.

[Review the text editors index](https://github.com/john-cd/rust_howto/issues/962)

</div>
