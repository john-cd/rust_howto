# File listing and display

{{#include filesystem_cli.incl.md}}

## `lsd` {#lsd}

[`lsd`][c-lsd-github]{{hi:lsd}} is a command-line utility written in Rust, a rewrite of the GNU `ls` command. It aims to provide a more visually appealing and informative directory listing with features like colored output, icons for different file types, `git` integration, tree-view, and recursive directory traversal.

```sh
apt install lsd
```

## `exa` {#exa}

[![exa-github][c-exa-github-badge]][c-exa-github]{{hi:exa}}

`exa` is a fast, colorful, and informative replacement for `ls`, offering improved defaults and a user-friendly experience. `exa` offers more extensive customization options compared to lsd. You can configure colors, icons, and output format in more detail with exa. [`lsd`][c-lsd]⮳{{hi:lsd}} uses a configuration file to store user preferences, while [`exa`][c-exa]⮳{{hi:exa}} primarily relies on command-line flags.

## `broot` {#broot}

[`broot`][c-broot-website]{{hi:broot}} [![broot-github][c-broot-github-badge]][c-broot-github] is a new way to see and navigate directory trees.

`broot` is an interactive directory tree explorer implemented in Rust, designed for efficient filesystem navigation and manipulation. It provides a navigable, hierarchical view of the [filesystem][p-filesystem], enabling rapid traversal via keyboard shortcuts and intuitive navigation. Beyond basic exploration, [`broot`][c-broot]⮳{{hi:broot}} integrates file management operations (move, copy, delete, rename) directly within its interface. Its shell integration allows seamless transitions between [`broot`][c-broot]⮳{{hi:broot}} and the shell, facilitating efficient workflow integration.

## `bat` {#bat}

[![bat-github][c-bat-github-badge]][c-bat-github]{{hi:bat}}

[`bat`][c-bat]⮳{{hi:bat}}{{hi:bat}} is a fast `cat` clone with syntax highlighting and Git{{hi:Git}} integration. It improves upon the standard `cat` command by providing colored output for various file types, line numbers, and integration with Git to show modifications. This enhances readability and makes it easier to quickly inspect code or configuration files directly in the terminal.

```sh
bat README.md
# Display multiple files at once
bat src/*.rs
# Read from stdin, determine the syntax automatically
curl -s https://sh.rustup.rs | bat
# Show and highlight non-printable characters:
bat -A /etc/hosts
```

## `open` {#open}

[![open][c-open-badge]][c-open]{{hi:open}}
[![open-crates.io][c-open-crates.io-badge]][c-open-crates.io]
[![open-github][c-open-github-badge]][c-open-github]
[![open-lib.rs][c-open-lib.rs-badge]][c-open-lib.rs]

`open` opens a path or [URL][p-url] using the program configured on the system. [`open`][c-open]⮳{{hi:open}} is a cross-platform command-line utility for opening files and URLs in their default associated applications. It abstracts away the platform-specific commands for opening files (like xdg-open on Linux, open on macOS, and start on Windows), providing a consistent and convenient way to launch files, websites, or directories from the terminal regardless of the operating system.

{{#include refs.incl.md}}
{{#include ../../refs/link-refs.md}}

<div class="hidden">
[filesystem: organize (P1)](https://github.com/john-cd/rust_howto/issues/237)
</div>
