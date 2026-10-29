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
# NASM is installed from its pinned, checksum-verified upstream archive below; Ninja is the build backend for CMake-based -sys crates.
winget install --id Ninja-build.Ninja -e --source winget --accept-source-agreements --accept-package-agreements
# Install the SDKs used by the Java, Ruby, and Python FFI examples.
winget install --id EclipseAdoptium.Temurin.21.JDK -e --source winget --accept-source-agreements --accept-package-agreements
winget install --id Python.Python.3.13 -e --source winget --accept-source-agreements --accept-package-agreements
winget install --id RubyInstallerTeam.RubyWithDevKit.3.3 -e --source winget --accept-source-agreements --accept-package-agreements
& (Join-Path $PSScriptRoot 'install_ruby_4_0.ps1')

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

# Install the pinned Oracle Instant Client runtime and SDK for OCI examples.
function Install-OracleInstantClient {
    $oracleVersion = $toolVersions['ORACLE_CLIENT_VERSION']
    $downloadId = $toolVersions['ORACLE_CLIENT_DOWNLOAD_ID']
    if ($oracleVersion -notmatch '^(?<major>\d+)\.(?<minor>\d+)\.') {
        throw "Invalid Oracle Instant Client version: $oracleVersion"
    }

    $oracleDirectoryName = "instantclient_$($Matches.major)_$($Matches.minor)"
    $oracleVersionDirectory = Join-Path $env:LOCALAPPDATA 'Programs'
    $oracleVersionDirectory = Join-Path $oracleVersionDirectory 'Oracle'
    $oracleVersionDirectory = Join-Path $oracleVersionDirectory 'InstantClient'
    $oracleVersionDirectory = Join-Path $oracleVersionDirectory $oracleVersion
    $oracleDirectory = Join-Path $oracleVersionDirectory $oracleDirectoryName
    $oracleDll = Join-Path $oracleDirectory 'oci.dll'
    $oracleSdkDirectory = Join-Path $oracleDirectory 'sdk'
    $oracleSdkDirectory = Join-Path $oracleSdkDirectory 'lib'
    $oracleSdkDirectory = Join-Path $oracleSdkDirectory 'msvc'
    $oracleImportLibrary = Join-Path $oracleSdkDirectory 'oci.lib'
    $oracleHeader = Join-Path (Join-Path $oracleDirectory 'sdk') 'include'
    $oracleHeader = Join-Path $oracleHeader 'oci.h'

    if ((Test-Path -LiteralPath $oracleDll -PathType Leaf) -and
        (Test-Path -LiteralPath $oracleImportLibrary -PathType Leaf) -and
        (Test-Path -LiteralPath $oracleHeader -PathType Leaf)) {
        Write-Host "Oracle Instant Client $oracleVersion is already installed." -ForegroundColor Green
    } else {
        if (Test-Path -LiteralPath $oracleDirectory) {
            throw "Oracle Instant Client directory exists without the required runtime and SDK files: $oracleDirectory"
        }

        $oracleStage = Join-Path $env:TEMP "oracle-instantclient-$oracleVersion-$PID"
        if (Test-Path -LiteralPath $oracleStage) {
            throw "Oracle Instant Client staging directory already exists: $oracleStage"
        }

        New-Item -ItemType Directory -Path $oracleStage | Out-Null
        try {
            $packages = @(
                @{
                    Name = "instantclient-basiclite-windows.x64-$oracleVersion.zip"
                    Sha256 = $toolVersions['ORACLE_CLIENT_WINDOWS_BASICLITE_SHA256']
                },
                @{
                    Name = "instantclient-sdk-windows.x64-$oracleVersion.zip"
                    Sha256 = $toolVersions['ORACLE_CLIENT_WINDOWS_SDK_SHA256']
                }
            )

            foreach ($package in $packages) {
                $archivePath = Join-Path $oracleStage $package.Name
                $archiveUri = "https://download.oracle.com/otn_software/nt/instantclient/$downloadId/$($package.Name)"
                Invoke-WebRequest -Uri $archiveUri -OutFile $archivePath
                $archiveHash = (Get-FileHash -LiteralPath $archivePath -Algorithm SHA256).Hash.ToLowerInvariant()
                if ($archiveHash -ne $package.Sha256) {
                    throw "Oracle Instant Client archive SHA-256 did not match for $($package.Name): $archiveHash"
                }
                Expand-Archive -LiteralPath $archivePath -DestinationPath $oracleStage -Force
            }

            $oracleSource = Join-Path $oracleStage $oracleDirectoryName
            foreach ($requiredFile in @(
                (Join-Path $oracleSource 'oci.dll'),
                (Join-Path (Join-Path (Join-Path $oracleSource 'sdk') 'lib') 'msvc\oci.lib'),
                (Join-Path (Join-Path $oracleSource 'sdk') 'include\oci.h')
            )) {
                if (-not (Test-Path -LiteralPath $requiredFile -PathType Leaf)) {
                    throw "Oracle Instant Client package is missing a required file: $requiredFile"
                }
            }

            New-Item -ItemType Directory -Path $oracleVersionDirectory -Force | Out-Null
            Move-Item -LiteralPath $oracleSource -Destination $oracleDirectory
        } finally {
            if (Test-Path -LiteralPath $oracleStage) {
                $tempRoot = [IO.Path]::GetFullPath($env:TEMP).TrimEnd([IO.Path]::DirectorySeparatorChar) + [IO.Path]::DirectorySeparatorChar
                $resolvedStage = [IO.Path]::GetFullPath($oracleStage)
                if (-not $resolvedStage.StartsWith($tempRoot, [StringComparison]::OrdinalIgnoreCase)) {
                    throw "Refusing to remove Oracle staging directory outside TEMP: $resolvedStage"
                }
                Remove-Item -LiteralPath $resolvedStage -Recurse -Force
            }
        }
    }

    $oracleSdkDirectory = Join-Path (Join-Path (Join-Path $oracleDirectory 'sdk') 'lib') 'msvc'
    [Environment]::SetEnvironmentVariable('ORACLE_HOME', $oracleDirectory, 'User')
    [Environment]::SetEnvironmentVariable('OCI_LIB_DIR', $oracleSdkDirectory, 'User')
    $env:ORACLE_HOME = $oracleDirectory
    $env:OCI_LIB_DIR = $oracleSdkDirectory
    Add-UserPath -Directory $oracleDirectory
}

Install-OracleInstantClient

# Refresh PATH and JAVA_HOME values installed by Winget for this PowerShell process.
$registeredPath = @(
    [Environment]::GetEnvironmentVariable('Path', 'Machine'),
    [Environment]::GetEnvironmentVariable('Path', 'User')
) | Where-Object { $_ }
$env:Path = (@($env:Path) + $registeredPath) -join ';'
$javaHome = [Environment]::GetEnvironmentVariable('JAVA_HOME', 'Machine')
if ([string]::IsNullOrWhiteSpace($javaHome)) {
    $javaHome = [Environment]::GetEnvironmentVariable('JAVA_HOME', 'User')
}
if ([string]::IsNullOrWhiteSpace($javaHome)) {
    $javaRoot = Join-Path $env:ProgramFiles 'Eclipse Adoptium'
    $javaHome = Get-ChildItem -LiteralPath $javaRoot -Directory -Filter 'jdk-21*' -ErrorAction SilentlyContinue |
        Sort-Object Name -Descending |
        Select-Object -First 1 -ExpandProperty FullName
}
if ($javaHome) {
    [Environment]::SetEnvironmentVariable('JAVA_HOME', $javaHome, 'User')
    $env:JAVA_HOME = $javaHome
    Add-UserPath -Directory (Join-Path $javaHome 'bin')
}

# Install the pinned NASM build required by aws-lc-sys.
$nasmVersion = $toolVersions["NASM_VERSION"]
$nasmDirectory = Join-Path $env:LOCALAPPDATA "Programs\NASM\$nasmVersion"
$nasmExecutable = Join-Path $nasmDirectory "nasm.exe"
if (-not (Test-Path -LiteralPath $nasmExecutable -PathType Leaf)) {
    if (Test-Path -LiteralPath $nasmDirectory) {
        throw "NASM install directory exists without nasm.exe: $nasmDirectory"
    }

    $nasmArchive = Join-Path $env:TEMP "nasm-$nasmVersion-win64-$PID.zip"
    $nasmUri = "https://www.nasm.us/pub/nasm/releasebuilds/$nasmVersion/win64/nasm-$nasmVersion-win64.zip"
    Invoke-WebRequest -Uri $nasmUri -OutFile $nasmArchive
    $nasmHash = (Get-FileHash -LiteralPath $nasmArchive -Algorithm SHA256).Hash.ToLowerInvariant()
    if ($nasmHash -ne $toolVersions["NASM_WINDOWS_SHA256"]) {
        throw "NASM archive SHA-256 did not match the pinned digest: $nasmHash"
    }

    New-Item -ItemType Directory -Path $nasmDirectory | Out-Null
    Add-Type -AssemblyName System.IO.Compression.FileSystem
    $archive = [System.IO.Compression.ZipFile]::OpenRead($nasmArchive)
    try {
        $archivePrefix = "nasm-$nasmVersion/"
        $installRoot = [IO.Path]::GetFullPath($nasmDirectory) + [IO.Path]::DirectorySeparatorChar
        foreach ($entry in $archive.Entries) {
            if (-not $entry.FullName.StartsWith($archivePrefix, [StringComparison]::Ordinal) -or [string]::IsNullOrEmpty($entry.Name)) {
                continue
            }

            $relativePath = $entry.FullName.Substring($archivePrefix.Length)
            $destination = [IO.Path]::GetFullPath((Join-Path $nasmDirectory $relativePath))
            if (-not $destination.StartsWith($installRoot, [StringComparison]::OrdinalIgnoreCase)) {
                throw "NASM archive entry escaped the install directory: $($entry.FullName)"
            }

            $parentDirectory = Split-Path -Parent $destination
            New-Item -ItemType Directory -Path $parentDirectory -Force | Out-Null
            [System.IO.Compression.ZipFileExtensions]::ExtractToFile($entry, $destination, $false)
        }
    } finally {
        $archive.Dispose()
    }

    Remove-Item -LiteralPath $nasmArchive
}
Add-UserPath -Directory $nasmDirectory
$env:Path = "$nasmDirectory;$env:Path"
& $nasmExecutable --version
@(
    (Join-Path $env:ProgramFiles 'LLVM\bin'),
    (Join-Path $env:ProgramFiles 'NASM'),
    'C:\Ruby33-x64\bin',
    'C:\Ruby33-x64\msys64\usr\bin',
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
    @{ Name = "cargo-fmt-toml"; Version = $toolVersions["CARGO_FMT_TOML_VERSION"] },
    @{ Name = "dprint"; Version = $toolVersions["DPRINT_VERSION"] },
    @{ Name = "rumdl"; Version = $toolVersions["RUMDL_VERSION"] },
    @{ Name = "flutter_rust_bridge_codegen"; Version = $toolVersions["FLUTTER_RUST_BRIDGE_CODEGEN_VERSION"] },
    @{ Name = "maturin"; Version = $toolVersions["MATURIN_VERSION"] }
)

foreach ($tool in $tools) {
    Write-Host "Installing $($tool.Name) $($tool.Version)..." -ForegroundColor Yellow
    $binstallArgs = @("--no-confirm", "--secure")
    if ($tool.Name -in @("cargo-fmt-toml", "dprint", "rumdl")) {
        $binstallArgs += "--force"
    }
    $binstallArgs += @("--version", $tool.Version, $tool.Name)
    cargo binstall @binstallArgs
}

$flutterVersion = $toolVersions["FLUTTER_VERSION"]
$flutterInstallPath = Join-Path $env:LOCALAPPDATA "Programs\Flutter\$flutterVersion"
& (Join-Path $PSScriptRoot "install_flutter_sdk.ps1") `
    -Version $flutterVersion `
    -GitRevision $toolVersions["FLUTTER_GIT_REVISION"] `
    -InstallPath $flutterInstallPath
$flutterBin = Join-Path $flutterInstallPath "bin"
Add-UserPath -Directory $flutterBin
$env:FLUTTER_ROOT = $flutterInstallPath
[Environment]::SetEnvironmentVariable('FLUTTER_ROOT', $flutterInstallPath, 'User')
$env:Path = "$flutterBin;$env:Path"

cargo install --git https://github.com/john-cd/mdbook-utils --rev $toolVersions["MDBOOK_UTILS_REV"] --locked mdbook-utils
cargo install --locked --force --version $toolVersions["MDBOOK_INDEXING_VERSION"] mdbook-indexing
cargo binstall --no-confirm --secure --version $toolVersions["MDBOOK_PANDOC_VERSION"] mdbook-pandoc
cargo binstall --no-confirm --secure mdbook-admonish

Write-Host "Environment check:" -ForegroundColor Cyan
just --version
mdbook --version
clang --version
nasm --version
if (-not (Test-Path -LiteralPath (Join-Path $env:OCI_LIB_DIR 'oci.lib') -PathType Leaf)) {
    throw "Oracle OCI import library not found in OCI_LIB_DIR: $env:OCI_LIB_DIR"
}
Write-Host "Oracle Instant Client SDK: $env:OCI_LIB_DIR"
pandoc --version | Select-Object -First 1
tectonic --version
cargo fmt-toml --version
dprint --version
rumdl --version
python --version
java --version
javac --version
ruby --version
& $env:RUBY --version
maturin --version
flutter --version
flutter_rust_bridge_codegen --version
Write-Host "Windows development environment ready!" -ForegroundColor Green
