# Windows build and test

This repository is designed to work on Windows with PowerShell.

1. Install Rust and required components:
   - Run `./scripts/install_deps.ps1` from an elevated PowerShell prompt.

2. Install `just` if needed:
   - `cargo install just`

3. Install native toolchain dependencies for Windows examples:
   - `winget install --id LLVM.LLVM -e --source winget`
   - ensure `clang.exe` and `llvm-config.exe` are on `PATH`
   - For GTK 4 GUI examples, install `pkg-config` and GTK 4 dev libraries via MSYS2:
     - `winget install --id MSYS2.MSYS2 -e --source winget`
     - Open the MSYS2 MinGW 64-bit shell and run:
       - `pacman -Syu`
       - `pacman -S mingw-w64-x86_64-pkg-config mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gdk-pixbuf2 mingw-w64-x86_64-cairo mingw-w64-x86_64-glib2`
     - Add `C:\msys64\mingw64\bin` to your `PATH` or set `PKG_CONFIG_PATH` to the MSYS2 pkg-config location.
     - Verify with `pkg-config --modversion gtk4`.

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
