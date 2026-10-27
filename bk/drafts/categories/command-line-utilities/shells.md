<!-- TODO Reviewed ??-??-2026 -->

# Shells and Related Utilities

{{#include shells.incl.md}}

## `starship` {#starship}

[![starship][c~starship~docs~badge]][c~starship~docs]{{hi:starship}}
[![starship~crates.io][c~starship~crates.io~badge]][c~starship~crates.io]
[![starship~repo][c~starship~repo~badge]][c~starship~repo]
[![starship~lib.rs][c~starship~lib.rs~badge]][c~starship~lib.rs]
[![cat~command-line-utilities][cat~command-line-utilities~badge]][cat~command-line-utilities]{{hi:Command line utilities}}

[`starship`][c~starship~repo]↗{{hi:starship}} is a fast, highly customizable prompt for any shell. It displays relevant environment info (Git branch, Node.js version, Rust version, etc.) concisely and visually. Starship is designed to be extensible, allowing users to configure the prompt to their specific needs and add support for various programming languages and tools. Its focus on performance ensures minimal overhead, even with a rich set of features.

Install it with Cargo, then add the initialization command for your shell to its startup file. This example configures Bash:

```bash
cargo install starship --locked
eval "$(starship init bash)"
```

## `nushell` {#nushell}

[Nushell][nushell~website]↗[(github)][nushell~repo]↗

[`nushell`][nushell~website]↗{{hi:nushell}} (or `Nu`) is a modern shell designed with a focus on structured data and a powerful, consistent language.

Unlike traditional shells that primarily handle text, `Nu` treats data as structured tables, enabling users to query, filter, and manipulate structured command output. Nu is written in Rust and has its own syntax and compatibility model; existing Bash or POSIX shell scripts do not automatically run unchanged in Nu.

Install Nu from Cargo, then launch it with `nu`. Cargo installs the main binary; Nu plugins are installed separately.

```bash
cargo install nu --locked
nu
```

## `fish` {#fish}

[fish][fish~repo]↗ is an interactive shell known for autosuggestions, syntax highlighting, and completions that work without extensive configuration. Fish 4 is implemented in Rust. Fish has its own scripting language, so scripts intended for POSIX shells should keep an explicit POSIX shebang.

Use your operating system's package manager. For example, on macOS install with Homebrew; on Ubuntu, the fish project documents its release PPA:

```bash
brew install fish

sudo apt-add-repository ppa:fish-shell/release-4
sudo apt update
sudo apt install fish
```

Start an interactive fish session with `fish`.

## `zoxide` {#zoxide}

[`zoxide`][c~zoxide~repo]↗ is a smarter `cd` command that learns frequently visited directories. Initialize it in the shell startup file before using `z`:

```bash
cargo install zoxide --locked
eval "$(zoxide init bash)"
z project
```

## `atuin` {#atuin}

[`Atuin`][atuin~repo]↗ replaces shell history with a local SQLite database and provides searchable history. History sync is optional; configure it only if you want to use a sync server.

Install using the project's installer, then search your local history:

```bash
curl --proto '=https' --tlsv1.2 -LsSf https://setup.atuin.sh | sh
atuin search cargo
```

## Shell Completion for Rust CLIs {#rust-cli-completions}

For a Rust CLI built with [`clap`][c~clap~docs]↗, use [`clap_complete`][c~clap_complete~docs]↗ to generate shell completions for the command. See [CLI argument parsing](../command-line-interface/argument_parsing.md) for the `clap` setup and completion examples.

```bash
cargo add clap_complete
```

## Related Topics {#related-topics .skip}

- [[command-line-interface | Command Line Interface]].
- [CLI argument parsing](../command-line-interface/argument_parsing.md).
- [[external_commands | External Commands]].

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
  [shells: expand](https://github.com/john-cd/rust_howto/issues/239)
</div>
