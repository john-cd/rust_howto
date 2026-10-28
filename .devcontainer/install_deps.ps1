# Purpose: Install the operating-system and Rust tools required by the development container.
# Usage: Run with pwsh -File .devcontainer/install_deps.ps1; no arguments are required.
#
# TODO Reviewed ??-??-2026.

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

$toolVersions = @{}
Get-Content (Join-Path $PSScriptRoot ".env") | ForEach-Object {
    if ($_ -match '^(?<name>[A-Z0-9_]+)="(?<version>[^"]+)"$') {
        $toolVersions[$Matches.name] = $Matches.version
    }
}

# Install Just (Task Runner)
winget install --id Casey.Just --exact --source winget --accept-source-agreements --accept-package-agreements
# Install LLVM/Clang (Required for bindgen in some crates)
winget install --id LLVM.LLVM -e --source winget --accept-source-agreements --accept-package-agreements
winget install --id SQLite.SQLite -e --source winget --accept-source-agreements --accept-package-agreements
# NASM is required to build aws-lc-sys; Ninja is the build backend for CMake-based -sys crates.
winget install --id NASM.NASM -e --source winget --accept-source-agreements --accept-package-agreements
winget install --id Ninja-build.Ninja -e --source winget --accept-source-agreements --accept-package-agreements

# Install the repository-pinned Pandoc release. Its MSI updates PATH when installed.
$pandocVersion = $toolVersions["PANDOC_VERSION"]
$pandocInstaller = Join-Path $env:TEMP "pandoc-$pandocVersion-windows-x86_64.msi"
$pandocUri = "https://github.com/jgm/pandoc/releases/download/$pandocVersion/pandoc-$pandocVersion-windows-x86_64.msi"
Invoke-WebRequest -Uri $pandocUri -OutFile $pandocInstaller
$pandocHash = (Get-FileHash -LiteralPath $pandocInstaller -Algorithm SHA256).Hash.ToLowerInvariant()
if ($pandocHash -ne $toolVersions["PANDOC_WINDOWS_MSI_SHA256"]) {
    throw "Pandoc MSI SHA-256 did not match the pinned digest: $pandocHash"
}
$quotedPandocInstaller = '"{0}"' -f $pandocInstaller
$pandocInstall = Start-Process -FilePath msiexec.exe -ArgumentList @('/i', $quotedPandocInstaller, '/qn', '/norestart') -Wait -PassThru
Remove-Item -LiteralPath $pandocInstaller -ErrorAction SilentlyContinue
if ($pandocInstall.ExitCode -notin @(0, 3010)) {
    throw "Pandoc installer failed with exit code $($pandocInstall.ExitCode)."
}

# Winget installs these tools outside the current shell's inherited PATH on some hosts.
function Add-UserPath {
    param([Parameter(Mandatory)][string]$Directory)

    if (-not (Test-Path -LiteralPath $Directory -PathType Container)) {
        return
    }

    $resolvedDirectory = (Resolve-Path -LiteralPath $Directory).Path.TrimEnd('\')
    $processEntries = @($env:Path -split ';' | Where-Object { $_ })
    if (-not ($processEntries | Where-Object { $_.TrimEnd('\') -ieq $resolvedDirectory })) {
        $env:Path = "$env:Path;$resolvedDirectory"
    }

    $userPath = [Environment]::GetEnvironmentVariable('Path', 'User')
    $userEntries = @($userPath -split ';' | Where-Object { $_ })
    if (-not ($userEntries | Where-Object { $_.TrimEnd('\') -ieq $resolvedDirectory })) {
        $newUserPath = if ([string]::IsNullOrWhiteSpace($userPath)) { $resolvedDirectory } else { "$userPath;$resolvedDirectory" }
        [Environment]::SetEnvironmentVariable('Path', $newUserPath, 'User')
    }
}

@(
    (Join-Path $env:ProgramFiles 'LLVM\bin'),
    (Join-Path $env:ProgramFiles 'NASM'),
    (Join-Path $env:LOCALAPPDATA 'Pandoc'),
    (Join-Path $env:LOCALAPPDATA 'bin'),
    (Join-Path $env:LOCALAPPDATA 'bin\NASM')
) | ForEach-Object { Add-UserPath -Directory $_ }

# Use the publisher's pinned prebuilt binary instead of compiling Tectonic's native libraries.
$tectonicVersion = $toolVersions["TECTONIC_VERSION"]
$tectonicArchive = Join-Path $env:TEMP "tectonic-$tectonicVersion-x86_64-pc-windows-msvc.zip"
$tectonicUri = "https://github.com/tectonic-typesetting/tectonic/releases/download/tectonic%40$tectonicVersion/tectonic-$tectonicVersion-x86_64-pc-windows-msvc.zip"
Invoke-WebRequest -Uri $tectonicUri -OutFile $tectonicArchive
$tectonicHash = (Get-FileHash -LiteralPath $tectonicArchive -Algorithm SHA256).Hash.ToLowerInvariant()
if ($tectonicHash -ne $toolVersions["TECTONIC_WINDOWS_SHA256"]) {
    throw "Tectonic archive SHA-256 did not match the pinned digest: $tectonicHash"
}
$tectonicDirectory = Join-Path $env:LOCALAPPDATA "Programs\Tectonic\$tectonicVersion"
New-Item -ItemType Directory -Force -Path $tectonicDirectory | Out-Null
Expand-Archive -LiteralPath $tectonicArchive -DestinationPath $tectonicDirectory -Force
Remove-Item -LiteralPath $tectonicArchive -ErrorAction SilentlyContinue
Add-UserPath -Directory $tectonicDirectory

# The GTK 4 and Relm4 examples are Linux-only in later/crates/cats/gui.

# Install cargo-binstall for Rust-based tools
Set-ExecutionPolicy Unrestricted -Scope Process; iex (iwr "https://raw.githubusercontent.com/cargo-bins/cargo-binstall/main/install-from-binstall-release.ps1").Content

Write-Host "Installing Rust-based tools..." -ForegroundColor Cyan
$tools = @(
    @{ Name = "mdbook"; Version = $toolVersions["MDBOOK_VERSION"] },
    @{ Name = "mdbook-linkcheck"; Version = $toolVersions["MDBOOK_LINKCHECK_VERSION"] },
    @{ Name = "cargo-nextest"; Version = $toolVersions["CARGO_NEXTEST_VERSION"] },
    @{ Name = "just"; Version = $toolVersions["JUST_VERSION"] },
    @{ Name = "cargo-sort"; Version = $toolVersions["CARGO_SORT_VERSION"] },
    @{ Name = "dprint"; Version = $toolVersions["DPRINT_VERSION"] },
    @{ Name = "rumdl"; Version = $toolVersions["RUMDL_VERSION"] }
)

foreach ($tool in $tools) {
    Write-Host "Installing $($tool.Name) $($tool.Version)..." -ForegroundColor Yellow
    $binstallArgs = @("--no-confirm", "--secure")
    if ($tool.Name -in @("dprint", "rumdl")) {
        $binstallArgs += "--force"
    }
    $binstallArgs += @("--version", $tool.Version, $tool.Name)
    cargo binstall @binstallArgs
}

cargo install --git https://github.com/john-cd/mdbook-utils --rev $toolVersions["MDBOOK_UTILS_REV"] --locked mdbook-utils
cargo install --locked --force --version $toolVersions["MDBOOK_INDEXING_VERSION"] mdbook-indexing
cargo binstall --no-confirm --secure --version $toolVersions["MDBOOK_PANDOC_VERSION"] mdbook-pandoc
cargo binstall --no-confirm --secure mdbook-admonish

Write-Host "Environment check:" -ForegroundColor Cyan
just --version
mdbook --version
clang --version
nasm --version
pandoc --version | Select-Object -First 1
tectonic --version
cargo sort --version
dprint --version
rumdl --version
Write-Host "Windows development environment ready!" -ForegroundColor Green
