<!-- TODO Reviewed ??-??-2026 -->

# Installing

{{#include install.incl.md}}

This page covers installing Rust command-line applications. To install the Rust compiler and manage Rust toolchains, start with [`rustup`][ex~installation~rustup].

| Topic                                    | Rust Crates                                                                                                                                                                                                                                                    |
| ---------------------------------------- | -------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Package Managers                         | [cargo][p~cargo]                                                                                                                                                                                                                                               |
| Installer Generators                     | [`cargo-deb`][c~cargo-deb~docs]↗{{hi:cargo-deb}}, [`cargo-rpm`][c~cargo-rpm~docs]↗{{hi:cargo-rpm}}, [`create-dmg`][create-dmg~repo]↗{{hi:create-dmg}} (for creating Debian packages, RPM packages, and [macOS][p~macos] disk [images][p~images], respectively) |
| Cross-Platform Installers                | [`tauri-bundler`][c~tauri-bundler~docs]↗{{hi:tauri-bundler}} (for bundling desktop applications)↗                                                                                                                                                              |
| Self-Updating                            | [`self_update`][c~self_update~docs]↗{{hi:self_update}}                                                                                                                                                                                                         |
| Build Systems (for installation scripts) | [`xtask`][c~xtask~docs]↗{{hi:xtask}} (for managing complex builds and installation-related tasks)                                                                                                                                                              |

## Install a Rust Binary with `cargo install` {#cargo-install}

[![cargo~website][c~cargo~website~badge]][c~cargo~website] [![cargo][c~cargo~docs~badge]][c~cargo~docs] [![cargo~crates.io][c~cargo~crates.io~badge]][c~cargo~crates.io] [![cargo~repo][c~cargo~repo~badge]][c~cargo~repo] [![cargo~lib.rs][c~cargo~lib.rs~badge]][c~cargo~lib.rs]{{hi:cargo}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

[`cargo install`][book~cargo~cargo-install]↗{{hi:cargo install}} builds and installs executable targets from a package. It is for tools you want available from a shell; it does not add a dependency to the current project's `Cargo.toml`. By default Cargo installs binaries from [`crates.io`][crates.io~website]↗{{hi:crates.io}} into the Cargo installation root's `bin` directory (usually `~/.cargo/bin` on Unix-like systems or `%USERPROFILE%\.cargo\bin` on Windows). Make sure that directory is on `PATH`.

```sh
# Install a published command-line tool using its packaged lockfile when present.
cargo install --locked ripgrep

# List and remove tools installed by Cargo.
cargo install --list
cargo uninstall ripgrep
```

Cargo can also install from a Git repository, a local path, or a named registry. Use `--version` to select a published version and `--root` to choose another installation directory. `cargo install` builds the package on your machine; `--locked` asks Cargo to use the package's published `Cargo.lock` when one is available. Some older packages do not include a lockfile, so `--locked` cannot pin their dependency resolution.

## Install a Rust Binary with `cargo binstall` {#cargo-binstall}

[![cargo-binstall][c~cargo-binstall~docs~badge]][c~cargo-binstall~docs] [![cargo-binstall~crates.io][c~cargo-binstall~crates.io~badge]][c~cargo-binstall~crates.io] [![cargo-binstall~repo][c~cargo-binstall~repo~badge]][c~cargo-binstall~repo] [![cargo-binstall~lib.rs][c~cargo-binstall~lib.rs~badge]][c~cargo-binstall~lib.rs]{{hi:cargo-binstall}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

`cargo binstall` is an optional tool for installing Rust binaries. It can use compatible prebuilt release artifacts instead of compiling the package locally, and can fall back to a source build when an artifact is unavailable, depending on its configuration.

Prebuilt artifacts are supplied through release infrastructure outside Cargo's normal source build. Check the project's release page and the [`cargo-binstall`][c~cargo-binstall~docs]↗{{hi:cargo-binstall}} documentation to understand where an artifact comes from and which fallback behavior is enabled before installing it.

## Related Topics {#related-topics .skip}

| Topic                                               | See also                           |
| --------------------------------------------------- | ---------------------------------- |
| Install and update the Rust compiler and toolchains | [`rustup`][ex~installation~rustup] |
| Manage project dependencies                         | [Cargo][p~cargo]                   |
| Find and compare crates                             | [Crate selection][p~crates]        |

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
  review
</div>
