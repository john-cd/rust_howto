<!-- TODO Reviewed ??-??-2026 -->

# Windows build and test

The `rust_howto`-owned Windows recipe shells use Git Bash. A few setup and index-generation helpers are PowerShell scripts invoked explicitly with `pwsh.exe`; see [the script support table](../bk/scripts/README.md) before invoking a platform-specific recipe.

## Install prerequisites

From the repository root, run:

```powershell
pwsh -File .devcontainer/install_deps.ps1
```

The script reads pinned tool versions from `.devcontainer/.env` and installs Rust components, `just`, `cargo-sort`, `dprint`, `rumdl`, LLVM/Clang, SQLite, NASM, Ninja, Pandoc, Tectonic, and the book tools with Winget, verified publisher downloads, and Cargo. It adds the LLVM, NASM, Pandoc, Tectonic, and local Rust tool directories to the current and user `PATH` when those directories exist. It does not take command-line parameters or install Git for Windows. Install Git for Windows separately; Git Bash must be available at `C:\Program Files\Git\bin\bash.exe`, the path used by the repository justfiles. For a custom installation, pass the desired path with just's `--shell` and `--shell-arg` options.

Use `.devcontainer/install_deps.ps1` for native Windows. `.devcontainer/install_deps.sh` is the separate Debian/Ubuntu installer; it uses `apt-get` and `sudo` and must run in Linux, WSL, or a Linux container. Do not run it through Git Bash on Windows.

The dev container builds `tools/`, which depends on the private `john-cd/book_tooling` repository. Authenticate with GitHub CLI and provide its token to BuildKit for the build:

```powershell
$env:GH_TOKEN = gh auth token --hostname github.com
docker compose --env-file .devcontainer/.env -f .devcontainer/compose.yaml build book
Remove-Item Env:GH_TOKEN
```

The token is used as a BuildKit secret only while Cargo fetches the private dependency; it is not stored in the image or `.devcontainer/.env`.

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

The `quick` recipe (`just q`) runs its Bash preview through Git Bash. `just spell` uses Typos, and `just lnk <pattern>` uses the Windows `mdbook-utils` binary. `just recipes fix_recipe_tables` uses Python, and `just refdefs sort` runs the shared Bash helper.

Run `just --list` for root recipes and modules, then `just <module> --list` for a module's recipes. Each imported module owns its shell settings and working directory; root settings do not automatically apply to it. Recipes tagged `[unix]` are unavailable on Windows.

## Native dependencies and validation

Full-workspace recipes build optional examples that depend on native libraries, SDKs, or external services. The installer covers common Windows tools, but some examples still need separately configured development libraries. A native linker error means the affected full gate has not passed; record the missing library and keep focused checks separate from the full gate.

Pingora validation requires a working Unix environment. On Windows, use WSL only when it is installed and starts successfully. For a single crate, run from `bk/crates`:

```powershell
cargo check --all-targets --all-features --locked
cargo test -p <crate> --all-targets
```

Some examples require external services and are marked with `#[ignore]` or `require_external_svc` tests.
