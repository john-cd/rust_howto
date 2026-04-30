<#
.SYNOPSIS
    Installs development dependencies for Windows.
#>

Write-Host "Installing Rust components..." -ForegroundColor Cyan
rustup default stable
rustup toolchain install nightly
rustup component add clippy
rustup component add rustfmt --toolchain nightly

Write-Host "Installing system tools via Winget..." -ForegroundColor Cyan
# Install Just (Task Runner)
winget install --id Casey.Just --exact
# Install LLVM/Clang (Required for bindgen in some crates)
winget install --id LLVM.LLVM -e --source winget
winget install --id SQLite.SQLite -e --source winget

# Install cargo-binstall for Rust-based tools
Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content

Write-Host "Installing Rust-based tools..." -ForegroundColor Cyan
$tools = @(
    "mdbook",
    "mdbook-admonish",
    "mdbook-linkcheck",
    "cargo-nextest",
    "mdbook-utils"
)

foreach ($tool in $tools) {
    Write-Host "Installing $tool..." -ForegroundColor Yellow
    cargo binstall --no-confirm $tool
}

Write-Host "Environment check:" -ForegroundColor Cyan
just --version
mdbook --version
clang --version
Write-Host "Windows development environment ready!" -ForegroundColor Green
