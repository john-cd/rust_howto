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

# Install MSYS2 to provide pkg-config and GTK4 development libraries on Windows
winget install --id MSYS2.MSYS2 -e --source winget
$msys2Path = 'C:\msys64\usr\bin\bash.exe'
if (Test-Path $msys2Path) {
    Write-Host "Installing MSYS2 GTK4 packages..." -ForegroundColor Cyan
    & $msys2Path -lc "pacman -Syu --noconfirm && pacman -S --noconfirm mingw-w64-x86_64-pkg-config mingw-w64-x86_64-gtk4 mingw-w64-x86_64-gdk-pixbuf2 mingw-w64-x86_64-cairo mingw-w64-x86_64-glib2"
} else {
    Write-Host "MSYS2 installation path not found at $msys2Path. Please install MSYS2 and GTK4 packages manually." -ForegroundColor Yellow
}

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
