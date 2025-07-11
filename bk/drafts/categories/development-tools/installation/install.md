# Installing

{{#include install.incl.md}}

| Topic | Rust Crates |
|---|---|
| Package Managers | [cargo][p~cargo] |
| Installer Generators | [`cargo-deb`][c~cargo_deb~docs]⮳{{hi:cargo-deb}}, [`cargo-rpm`][c~cargo_rpm~docs]⮳{{hi:cargo-rpm}}, `create-dmg` (for creating Debian packages, RPM packages, and [macOS][p~macos] disk [images][p~images], respectively) |
| Cross-Platform Installers | `tauri-bundler` (for bundling desktop applications) |
| Self-Updating | `self_update` |
| Build Systems (for installation scripts) | xtask (for managing complex builds and installation-related tasks) |

## Build and Install a Rust Binary with `cargo install` {#cargo-install}

[![cargo~website][c~cargo~website~badge]][c~cargo~website] [![cargo][c~cargo~docs~badge]][c~cargo~docs] [![cargo~crates.io][c~cargo~crates.io~badge]][c~cargo~crates.io] [![cargo~github][c~cargo~github~badge]][c~cargo~github] [![cargo~lib.rs][c~cargo~lib.rs~badge]][c~cargo~lib.rs]{{hi:cargo}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

The `cargo install` command manages [`cargo`][c~cargo~docs]⮳{{hi:cargo}}'s local set of installed binary [crates][p~crates]. Only packages which have executable{{hi:Executables}} `[[bin]]` or `[[example]]` targets can be installed, and all executables are installed into the installation root's bin folder. By default only binaries, not examples, are installed. There are multiple sources from which a crate can be installed. The default source location is [`crates.io`][crates.io~website]{{hi:crates.io}}⮳, but the --git, --path, and --registry flags can change this source. This command operates on system or user level, not project level.

## Install a Rust Binary with `cargo binstall` {#cargo-binstall}

[![cargo-binstall][c~cargo_binstall~docs~badge]][c~cargo_binstall~docs] [![cargo-binstall~crates.io][c~cargo_binstall~crates.io~badge]][c~cargo_binstall~crates.io] [![cargo-binstall~github][c~cargo_binstall~github~badge]][c~cargo_binstall~github] [![cargo-binstall~lib.rs][c~cargo_binstall~lib.rs~badge]][c~cargo_binstall~lib.rs]{{hi:cargo-binstall}} [![cat~development-tools][cat~development-tools~badge]][cat~development-tools]{{hi:Development tools}}

`cargo binstall` provides a low-complexity mechanism for installing Rust binaries{{hi:Rust binaries installation}} as an alternative to building from source (e.g. via `cargo install`) or manually downloading packages. This is intended to work with existing CI artifacts and infrastructure, and with minimal overhead for package maintainers.

`cargo binstall` works by fetching the crate information from [`crates.io`][crates.io~website]{{hi:crates.io}}⮳ and searching the linked repository for matching releases and artifacts, falling back to the `quickinstall`{{hi:quickinstall}} third-party artifact host, to alternate targets as supported, and finally to `cargo install` as a last resort.

{{#include refs.incl.md}}
{{#include ../../../refs/link-refs.md}}

<div class="hidden">
[review NOW](https://github.com/john-cd/rust_howto/issues/918)
</div>
