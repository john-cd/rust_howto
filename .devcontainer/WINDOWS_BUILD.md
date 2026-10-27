<!-- TODO Reviewed ??-??-2026 -->

# Windows build and test

The repository's Windows recipes use PowerShell 7 at the entry point. Some imported book modules run Git Bash recipes; see [the script support table](../bk/scripts/README.md) before invoking a platform-specific recipe.

## Install prerequisites

From the repository root, run:

```powershell
pwsh -File .devcontainer/install_deps.ps1
```

The script reads pinned tool versions from `.devcontainer/.env` and installs Rust components, `just`, LLVM/Clang, SQLite, NASM, Ninja, and the book tools with Winget and Cargo. It does not take command-line parameters. If Git for Windows is installed outside `C:\Program Files\Git`, set `GIT_BASH` to the full path of `bash.exe`.

Run the repository build setup once per checkout:

```powershell
just build-setup
```

## Run book recipes

From `bk`, the main workspace recipes are:

```powershell
just cka  # Check all code with all features.
just ca   # Run Clippy across the workspace.
just ba   # Build all code with all features.
just nta  # Run all tests, including doctests.
just bb   # Build the book.
just s    # Serve the book locally.
```

The `quick` recipe (`just q`) runs its Bash preview through Git Bash. `just spell` uses Typos, and `just lnk <pattern>` uses the Windows `mdbook-utils` binary. `just recipes fix_recipe_tables` and `just refdefs sort` also have native Windows implementations.

Run `just --list` for root recipes and modules, then `just <module> --list` for a module's recipes. Each imported module owns its shell settings and working directory; root settings do not automatically apply to it. Recipes tagged `[unix]` are unavailable on Windows.

## Native dependencies and validation

Full-workspace recipes build optional examples that depend on native libraries, SDKs, or external services. The installer covers common Windows tools, but some examples still need separately configured development libraries. A native linker error means the affected full gate has not passed; record the missing library and keep focused checks separate from the full gate.

Pingora validation requires a working Unix environment. On Windows, use WSL only when it is installed and starts successfully. For a single crate, run from `bk/crates`:

```powershell
cargo check --all-targets --all-features --locked
cargo test -p <crate> --all-targets
```

Some examples require external services and are marked with `#[ignore]` or `require_external_svc` tests.
