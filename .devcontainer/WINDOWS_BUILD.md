# Windows build and test

This repository is designed to work on Windows with PowerShell.

1. Install Rust and required components:
   - Run `./scripts/install_deps.ps1` from an elevated PowerShell prompt.

2. Install `just` if needed:
   - `cargo install just`

3. Install native toolchain dependencies for Windows examples:
   - `winget install --id LLVM.LLVM -e --source winget`
   - ensure `clang.exe` and `llvm-config.exe` are on `PATH`

4. Run the book workspace commands from `bk`:
   - `cd bk`
   - `just cka` — check all code with all features
   - `just ca` — run clippy across the workspace
   - `just ba` — build all code (all features)
   - `just nta` — run all tests including doctests

5. If you want to compile or test a single crate directly:
   - `cd bk/crates`
   - `cargo check --all-targets --all-features --locked`
   - `cargo test -p <crate> --all-targets`

6. Example-specific notes:
   - Some examples require native system libraries or external services.
   - Examples that need external services are often tagged with `#[ignore]` or `require_external_svc` tests.
