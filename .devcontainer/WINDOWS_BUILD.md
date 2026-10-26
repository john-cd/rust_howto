<!-- TODO Reviewed ??-??-2026 -->

# Windows build and test

This repository is designed to work on Windows with PowerShell.

1. Install Rust and required components:
   - Run `./scripts/install_deps.ps1` from an elevated PowerShell prompt.

2. Install `just` if needed:
   - The repository pins `just` in `.devcontainer/.env`; run `./scripts/install_deps.ps1` to install that version.
   - If installing it separately, use `cargo install just --version 1.59.0 --locked`.

3. Install native toolchain dependencies for Windows examples:
   - `winget install --id LLVM.LLVM -e --source winget`
   - ensure `clang.exe` and `llvm-config.exe` are on `PATH`
   - The staging GTK 4 and Relm4 examples are excluded on Windows; GTK and this repository's MSVC toolchain require separate platform setup. See the GUI chapter for GTK instructions.

4. Run the book workspace commands from `bk`:
   - `cd bk`
   - `just cka` — check all code with all features
   - `just ca` — run clippy across the workspace
   - `just ba` — build all code (all features)
   - `just nta` — run all tests including doctests
   - `just bb` — build the book; `just s` — serve it locally

The root `bk/justfile` uses PowerShell 7 on Windows. Each imported module has
its own shell setting: several modules use Git Bash from Git for Windows, while
others use PowerShell. A recipe tagged `[unix]` is unavailable on Windows;
recipes with a `[windows]` implementation or no platform tag can be listed and
run there. See `bk/scripts/README.md` for the module support table. The book's
`quick` recipe (`just q`) runs its Bash preview through Git Bash. `just spell`
uses Typos and `just lnk <pattern>` uses the Windows `mdbook-utils` binary; both
are available on Windows. Set `GIT_BASH` if Git for Windows is installed outside
`C:\Program Files\Git`.

From `bk`, `just recipes fix_recipe_tables` and `just refdefs sort` also have
native Windows implementations. Run `just --list` to see root recipes and
modules, then `just <module>` to see that module's recipes. Module settings and
default working directories belong to each module's `mod.just`; root settings
do not automatically apply to them. Commands that need to run relative to the
repository root use an explicit root path.

5. If you want to compile or test a single crate directly:
   - `cd bk/crates`
   - `cargo check --all-targets --all-features --locked`
   - `cargo test -p <crate> --all-targets`

6. Example-specific notes:
   - Some examples require native system libraries or external services.
   - Examples that need external services are often tagged with `#[ignore]` or `require_external_svc` tests.
