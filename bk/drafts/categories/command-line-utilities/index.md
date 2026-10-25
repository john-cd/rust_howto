<!-- TODO Reviewed ??-??-2026 -->

# Command-line Utilities Written in Rust

[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

This index collects Rust command-line programs that replace familiar utilities or provide related tools. For larger applications written in Rust, see the [[written_in_rust | Software Built in Rust]] overview.

- [`ripgrep`][c~ripgrep~docs]↗{{hi:ripgrep}} (`rg`): A fast, recursive text searcher and a common alternative to `grep`.
- [`fd`][c~fd-find~docs]↗{{hi:fd}}: A user-friendly file finder often used instead of `find`.
- [`bat`][c~bat~docs]↗{{hi:bat}}: A `cat` alternative that adds syntax highlighting, Git integration, and paging.
- [`eza`][eza~repo]↗{{hi:eza}}: A maintained `ls` alternative with tree views and file metadata.
- [`delta`][c~delta~docs]↗{{hi:delta}}: A syntax-highlighting pager for Git diffs, commonly used in place of `less` for diff output.
- [`hyperfine`][c~hyperfine~docs]↗{{hi:hyperfine}}: A command-line benchmarking tool for comparing program runtimes.
- [`just`][c~just~docs]↗{{hi:just}}: A project command runner for named recipes, similar to Make but with its own concise syntax.
- [`zoxide`][c~zoxide~docs]↗{{hi:zoxide}}: A smarter `cd` command that jumps to frequently used directories.
- [`bottom`][c~bottom~docs]↗{{hi:bottom}}: A customizable, cross-platform system and process monitor in the style of `top` or `htop`.
- [`dust`][c~dust~docs]↗{{hi:dust}}: A visual disk-usage analyzer as an alternative to `du`.
- [`sd`][c~sd~docs]↗{{hi:sd}}: A simpler, user-friendly alternative to `sed` for find-and-replace operations.
- [`xplr`][c~xplr~docs]↗{{hi:xplr}}: A customizable terminal file manager for interactive directory navigation.
- [`navi`][c~navi~docs]↗{{hi:navi}}: An interactive shell cheatsheet for discovering and running commands.

## Filesystem Utilities

{{#include filesystem_cli.incl.md}}

## Networking Utilities

{{#include networking_cli.incl.md}}

## Shells

{{#include shells.incl.md}}

## References

- [My terminal became more Rusty][blog~rusty-terminal]↗.

## Related Topics

- [[command-line-interface | Command Line Interface]].
- [[external_commands | External Commands]].
- [[links | Links]].
- [[written_in_rust | Written in Rust]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}
