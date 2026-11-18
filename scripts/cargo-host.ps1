# TODO Reviewed ??-??-2026.

<#
.SYNOPSIS
Configures shared Cargo defaults or links checkout-local Cargo output directories.

.DESCRIPTION
Use -LinkOutputs to connect this checkout's target, build, and sccache paths to the
configured central roots. Use -ConfigureHost to update shared compiler defaults in
CARGO_HOME/config.toml. The switches can be combined.

.PARAMETER BuildRoot
Base directory for per-repository, per-slot Cargo intermediate build files.
-LinkOutputs uses it to form the checkout's build link. The argument remains required
even when only -ConfigureHost is selected.

.PARAMETER ConfigureHost
Sets the shared rustc-wrapper and incremental defaults in CARGO_HOME/config.toml,
while removing global target-dir and build-dir overrides.

.PARAMETER LinkOutputs
Creates the checkout's target, build, and .sccache links using the configured roots.
Existing files, directories, or links to another destination cause a terminating error.

.EXAMPLE
pwsh -File .\scripts\cargo-host.ps1 -BuildRoot D:\code_build -LinkOutputs
Creates or verifies the main checkout's Cargo output links.

.EXAMPLE
pwsh -File .\scripts\cargo-host.ps1 -BuildRoot D:\code_build -ConfigureHost
Updates the shared Cargo compiler defaults for the current CARGO_HOME.
#>

param(
    [Parameter(Mandatory)][string]$BuildRoot,
    [switch]$ConfigureHost,
    [switch]$LinkOutputs
)
# Stop on path, permission, and config-write errors instead of leaving partial setup.
$ErrorActionPreference = 'Stop'
# Normalize the root used by -LinkOutputs before forming link destinations.
$fullBuildRoot = [IO.Path]::GetFullPath($BuildRoot)
if ($IsWindows) {
    $existing = $fullBuildRoot
    while (-not (Test-Path -LiteralPath $existing)) {
        $parent = Split-Path -Parent $existing
        if (-not $parent -or $parent -eq $existing) { throw "Cannot resolve build volume: $BuildRoot" }
        $existing = $parent
    }
    # Native drive information avoids starting a CIM query on every runner call.
    # Resolve mount points/junctions through the volume API when a parent is a link.
    $driveRoot = [IO.Path]::GetPathRoot($existing)
    $fileSystem = [IO.DriveInfo]::new($driveRoot).DriveFormat
    $ancestor = $existing
    while ($ancestor -and $ancestor -ne $driveRoot) {
        if ((Get-Item -LiteralPath $ancestor -Force).Attributes -band [IO.FileAttributes]::ReparsePoint) {
            $fileSystem = (Get-Volume -FilePath $existing -ErrorAction Stop).FileSystem
            break
        }
        $ancestor = Split-Path -Parent $ancestor
    }
    if ($fileSystem -notin @('NTFS', 'ReFS')) {
        throw "Cargo intermediate builds require NTFS or ReFS with Rust 1.100.0 or newer; $BuildRoot uses $fileSystem."
    }
}

# Resolve this checkout's central output slot for Cargo and rust-analyzer.
if ($LinkOutputs) {
    $checkout = Split-Path -Parent $PSScriptRoot
    $parent = Split-Path -Parent $checkout
    $slot = if ((Split-Path -Leaf $parent) -eq '.worktrees') { Split-Path -Leaf $checkout } else { 'main' }
    if ($slot -ne 'main' -and $slot -cnotmatch '^[abc]$') { throw 'Use only .worktrees/a, .worktrees/b, or .worktrees/c.' }
    $repo = if ($slot -eq 'main') { Split-Path -Leaf $checkout } else { Split-Path -Leaf (Split-Path -Parent $parent) }
    $cacheHome = if ($env:XDG_CACHE_HOME) { $env:XDG_CACHE_HOME } else { Join-Path $HOME '.cache' }
    $targetRoot = if ($env:CODE_TARGET_ROOT) { $env:CODE_TARGET_ROOT } elseif ($IsWindows) { 'D:/target' } else { Join-Path $cacheHome 'cargo-target' }
    $cacheRoot = if ($env:CODE_SCCACHE_ROOT) { $env:CODE_SCCACHE_ROOT } elseif ($env:SCCACHE_DIR) { $env:SCCACHE_DIR } elseif ($IsWindows) { 'D:/cache/sccache' } else { Join-Path $cacheHome 'sccache' }
    $outputs = [ordered]@{
        target = Join-Path (Join-Path $targetRoot $repo) $slot
        build = Join-Path (Join-Path $fullBuildRoot $repo) $slot
        '.sccache' = $cacheRoot
    }
    foreach ($name in $outputs.Keys) {
        $destination = [IO.Path]::GetFullPath($outputs[$name])
        $link = Join-Path $checkout $name
        $item = Get-Item -LiteralPath $link -Force -ErrorAction SilentlyContinue
        if ($null -ne $item) {
            # Preserve user data and only accept an existing link to the exact expected root.
            if (-not ($item.Attributes -band [IO.FileAttributes]::ReparsePoint)) { throw "Refusing to replace existing directory or file: $link" }
            $existingTarget = [IO.Path]::GetFullPath($item.ResolveLinkTarget($true).FullName)
            if ($existingTarget -ne $destination) { throw "Refusing to replace $link -> $existingTarget (expected $destination)" }
        } else {
            # Create the shared destination before adding a junction on Windows or symlink elsewhere.
            New-Item -ItemType Directory -Path $destination -Force | Out-Null
            $kind = if ($IsWindows) { 'Junction' } else { 'SymbolicLink' }
            New-Item -ItemType $kind -Path $link -Target $destination | Out-Null
        }
    }
}

if ($ConfigureHost) {
    $packages = if ($env:CODE_PACKAGES_ROOT) { $env:CODE_PACKAGES_ROOT } elseif ($IsWindows) { 'D:/packages' } else { Join-Path $HOME '.local/share' }
    $cargoHome = if ($env:CARGO_HOME) { $env:CARGO_HOME } else { Join-Path $packages 'cargo' }
    $configPath = Join-Path $cargoHome 'config.toml'
    New-Item -ItemType Directory -Path $cargoHome -Force | Out-Null
    $text = if (Test-Path -LiteralPath $configPath) { [IO.File]::ReadAllText($configPath).Replace("`r`n", "`n") } else { '' }
    # Keep shared compiler defaults in CARGO_HOME; checkout-local config and links own output paths.
    $values = [ordered]@{
        'rustc-wrapper' = 'sccache'
        'incremental' = 'false'
    }
    if ($text -notmatch '(?m)^\[build\]\s*$') { $text = $text.TrimEnd() + "`n`n[build]`n" }
    $match = [regex]::Match($text, '(?ms)^\[build\][^\n]*\n(?<body>.*?)(?=^\[|\z)')
    $body = $match.Groups['body'].Value
    # Remove only global output paths so they cannot override this checkout's links.
    foreach ($key in @('target-dir', 'build-dir')) {
        $pattern = '(?m)^[ \t]*' + [regex]::Escape($key) + '[ \t]*=.*(?:\n|$)'
        $body = [regex]::Replace($body, $pattern, '')
    }
    foreach ($key in $values.Keys) {
        if ($values[$key].Contains("'")) { throw "Cargo roots cannot contain a single quote: $($values[$key])" }
        $line = if ($key -eq 'incremental') { "$key = false" } else { "$key = '$($values[$key])'" }
        $pattern = '(?m)^[ \t]*' + [regex]::Escape($key) + '[ \t]*=.*$'
        if ([regex]::IsMatch($body, $pattern)) { $body = [regex]::Replace($body, $pattern, [System.Text.RegularExpressions.MatchEvaluator]{ param($m) $line }) }
        else { $body = $body.TrimEnd() + "`n$line`n" }
    }
    $replacement = "[build]`n" + $body.TrimStart("`n")
    # Preserve unrelated Cargo settings while replacing only the edited build section.
    $text = $text.Substring(0, $match.Index) + $replacement + $text.Substring($match.Index + $match.Length)
    [IO.File]::WriteAllText($configPath, $text.TrimStart("`n").TrimEnd() + "`n", [Text.UTF8Encoding]::new($false))
    Write-Output "Configured $configPath"
} else { Write-Output 'ok' }
