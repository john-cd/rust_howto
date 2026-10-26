# Purpose: Install the operating-system and Rust tools required by the development container.
# Usage: Run with pwsh -File .devcontainer/install_deps.ps1; pass any required parameters.
#
# TODO Reviewed ??-??-2026

<#
.SYNOPSIS
    Installs development dependencies for Windows.
#>

Write-Host "Installing Rust components..." -ForegroundColor Cyan
$rustfmtToolchain = if ($env:RUSTFMT_TOOLCHAIN) {
    $env:RUSTFMT_TOOLCHAIN
} else {
    (Select-String -Path (Join-Path $PSScriptRoot "..\rustfmt-toolchain.toml") -Pattern '^channel\s*=\s*"([^"]+)"').Matches[0].Groups[1].Value
}
rustup default stable
rustup toolchain install $rustfmtToolchain
rustup component add clippy
rustup component add rustfmt --toolchain $rustfmtToolchain

Write-Host "Installing system tools via Winget..." -ForegroundColor Cyan
# Install Just (Task Runner)
winget install --id Casey.Just --exact
# Install LLVM/Clang (Required for bindgen in some crates)
winget install --id LLVM.LLVM -e --source winget
winget install --id SQLite.SQLite -e --source winget
# NASM is required to build aws-lc-sys; Ninja is the build backend for CMake-based -sys crates.
# If `nasm` is not found afterwards, add its install folder to PATH and restart the terminal.
winget install --id NASM.NASM -e --source winget
winget install --id Ninja-build.Ninja -e --source winget

# The GTK 4 and Relm4 examples are Linux-only in later/crates/cats/gui.

# Install cargo-binstall for Rust-based tools
Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content

Write-Host "Installing Rust-based tools..." -ForegroundColor Cyan
$toolVersions = @{}
Get-Content (Join-Path $PSScriptRoot ".env") | ForEach-Object {
    if ($_ -match '^(?<name>[A-Z_]+)="(?<version>[^"]+)"$') {
        $toolVersions[$Matches.name] = $Matches.version
    }
}

$tools = @(
    @{ Name = "mdbook"; Version = $toolVersions["MDBOOK_VERSION"] },
    @{ Name = "mdbook-linkcheck"; Version = $toolVersions["MDBOOK_LINKCHECK_VERSION"] },
    @{ Name = "cargo-nextest"; Version = $toolVersions["CARGO_NEXTEST_VERSION"] },
    @{ Name = "just"; Version = $toolVersions["JUST_VERSION"] }
)

foreach ($tool in $tools) {
    Write-Host "Installing $($tool.Name) $($tool.Version)..." -ForegroundColor Yellow
    cargo binstall --no-confirm --version $tool.Version $tool.Name
}

cargo install --git https://github.com/john-cd/mdbook-utils --rev $toolVersions["MDBOOK_UTILS_REV"] --locked mdbook-utils
cargo install --locked --force --version $toolVersions["MDBOOK_INDEXING_VERSION"] mdbook-indexing
cargo binstall --no-confirm --version $toolVersions["MDBOOK_PANDOC_VERSION"] mdbook-pandoc
cargo binstall --no-confirm --version $toolVersions["TECTONIC_VERSION"] tectonic
cargo binstall --no-confirm mdbook-admonish

Write-Host "Environment check:" -ForegroundColor Cyan
just --version
mdbook --version
clang --version
Write-Host "Windows development environment ready!" -ForegroundColor Green
