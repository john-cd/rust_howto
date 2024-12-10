# Manage and build code

{{#include code_build.incl.md}}

## Save and run project-specific commands with the `just` command runner {#just}

[![just][c-just-badge]][c-just] [![just-crates.io][c-just-crates.io-badge]][c-just-crates.io] [![just-github][c-just-github-badge]][c-just-github] [![just-lib.rs][c-just-lib.rs-badge]][c-just-lib.rs]{{hi:just}}{{hi:Command-line}}{{hi:Development}}{{hi:Runner}}{{hi:Task}}{{hi:Utility}} [![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}} [![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}

`just`{{hi:just}} is a command runner{{hi:Command runner}}. It is a Rust-based equivalent to `make` without the ability to detect file changes, but with significantly fewer syntactic warts.

Consult the [Just programmer's manual][c-just-programmer-manual]⮳.

### Example `justfile`

Place it in the root folder of your project. Run [`just`][c-just-website]{{hi:just}} to see a list of recipes. Run `just <recipe>` to execute the corresponding recipe.

```makefile
# Load a .env file, if present.
set dotenv-load

default:
 @just --list --unsorted

# Check a local package and all of its dependencies for errors
check:
 @cargo check

# Compile a local package and all of its dependencies
build: check
 @cargo build

# Run a binary or example of the local packagels
run: check
 @cargo run

system-info:
 @echo "This is an {{arch()}} machine".

# Shebang script example
foo:
 #!/usr/bin/env bash
 set -euxo pipefail
 hello='Yo'
 echo "$hello from Bash!"
```

### Install `just` in a dev container

```Dockerfile
FROM mcr.microsoft.com/devcontainers/base:bullseye
# or perhaps mcr.microsoft.com/devcontainers/rust:bullseye if you want rust & cargo

SHELL ["bash", "-c"]

# Prerequisites to install Just: https://just.systems/man/en
RUN <<EOF
 wget -qO - 'https://proget.makedeb.org/debian-feeds/prebuilt-mpr.pub' | gpg --dearmor | sudo tee /usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg 1> /dev/null
 echo "deb [arch=all,$(dpkg --print-architecture) signed-by=/usr/share/keyrings/prebuilt-mpr-archive-keyring.gpg] https://proget.makedeb.org prebuilt-mpr $(lsb_release -cs)" | sudo tee /etc/apt/sources.list.d/prebuilt-mpr.list
 sudo apt update
EOF

RUN apt-get update && export DEBIAN_FRONTEND=noninteractive \
  && apt-get -y install just \
  && apt-get autoremove -y && apt-get clean -y
```

For Alpine, use [`apk`][apk-website]{{hi:apk}}⮳:

```dockerfile
# Just: https://just.systems/man/en/
RUN apk add just
```

## Check your Rust code in the background {#bacon}

[![bacon][c-bacon-badge]][c-bacon]{{hi:bacon}}
[![bacon-crates.io][c-bacon-crates.io-badge]][c-bacon-crates.io]
[![bacon-github][c-bacon-github-badge]][c-bacon-github]
[![bacon-lib.rs][c-bacon-lib.rs-badge]][c-bacon-lib.rs]
[![cat-development-tools][cat-development-tools-badge]][cat-development-tools]{{hi:Development tools}}
[![cat-command-line-utilities][cat-command-line-utilities-badge]][cat-command-line-utilities]{{hi:Command line utilities}}

`bacon` is a background rust code checker. It is designed for minimal interaction, so that you can just let it run, alongside your editor, and be notified of warnings, errors, or test failures in your Rust code.

```sh
# Install or update `bacon`
cargo install --locked bacon

# Check the current project
bacon

# Run clippy instead of cargo check
bacon clippy
```

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
(See cargo-make if you want something with a bulkier syntax but more cross-platform portability)
</div>