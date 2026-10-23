# TODO Reviewed ??-??-2026
<#
.SYNOPSIS
    Installs development dependencies for Windows.
#>

Write-Host "Installing Rust components..." -ForegroundColor Cyan
rustup default stable
rustup toolchain install nightly-2026-10-22
rustup component add clippy
rustup component add rustfmt --toolchain nightly-2026-10-22

Write-Host "Installing system tools via Winget..." -ForegroundColor Cyan
# Install Just (Task Runner)
winget install --id Casey.Just --exact
# Install LLVM/Clang (Required for bindgen in some crates)
winget install --id LLVM.LLVM -e --source winget
winget install --id SQLite.SQLite -e --source winget

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
