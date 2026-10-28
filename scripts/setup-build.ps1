# Purpose: Set up this checkout's external Cargo build paths and compiler-cache links.
# Usage: Run with pwsh -File scripts/setup-build.ps1; pass any required parameters.
#
# TODO Reviewed ??-??-2026.

param(
    [ValidatePattern('^[a-z]$')][string]$Slot,
    [string]$Branch,
    [switch]$RestartCache,
    [string]$ExistingBaseDirs = $env:SCCACHE_BASEDIRS,
    [string]$TargetDir,
    [string]$BuildDir
)

$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent $PSScriptRoot
$commonGitDir = (& git -C $repo rev-parse --path-format=absolute --git-common-dir)
if ($LASTEXITCODE) { throw 'Could not locate the shared Git directory.' }
$mainRepo = Split-Path -Parent $commonGitDir.Trim()

function Get-CacheBaseDirs {
    $configuredBases = @(
        $ExistingBaseDirs,
        [Environment]::GetEnvironmentVariable('SCCACHE_BASEDIRS', 'User'),
        [Environment]::GetEnvironmentVariable('SCCACHE_BASEDIRS', 'Machine')
    ) -join ';'
    $bases = @($configuredBases -split ';' | Where-Object { $_ })
    $bases += $mainRepo.Replace('\', '/')
    $bases += 'a'..'z' | ForEach-Object { (Join-Path $mainRepo ".worktrees/$_").Replace('\', '/') }
    $worktreePaths = @(& git -C $mainRepo worktree list --porcelain | ForEach-Object {
        if ($_ -match '^worktree (.+)$') { $Matches[1].Replace('\', '/') }
    })
    if ($LASTEXITCODE) { throw 'Could not enumerate registered Git worktrees.' }
    $bases += $worktreePaths
    return $bases | Select-Object -Unique
}

if ($RestartCache) {
    if (Get-Process cargo,rustc -ErrorAction SilentlyContinue) {
        throw 'Wait for running Cargo/rustc processes before restarting the shared sccache daemon.'
    }
    # sccache reads BASEDIRS when its daemon starts, not on each client request.
    # Retain other projects' bases, reusable slots, and every registered worktree.
    $env:SCCACHE_BASEDIRS = (Get-CacheBaseDirs) -join ';'
    & sccache --stop-server
    & sccache --start-server
    if ($LASTEXITCODE) { throw 'sccache startup failed.' }
    & sccache --show-stats
    exit $LASTEXITCODE
}

if ($Slot) {
    if (-not $Branch) { throw 'Specify a branch for the worktree slot.' }
    $checkout = Join-Path $mainRepo ".worktrees/$Slot"
    if (Test-Path -LiteralPath $checkout) {
        throw "Slot $Slot already exists. Inspect git worktree list and its status, then reuse it without deleting it."
    }
    & git -C $mainRepo check-ref-format --branch $Branch
    if ($LASTEXITCODE) { throw 'Invalid branch name.' }
    & git -C $mainRepo show-ref --verify --quiet "refs/heads/$Branch"
    if ($LASTEXITCODE -eq 0) {
        & git -C $mainRepo worktree add $checkout $Branch
    } else {
        & git -C $mainRepo worktree add -b $Branch $checkout HEAD
    }
    if ($LASTEXITCODE) { throw 'Worktree creation failed.' }
    $repo = $checkout
    # Do not inherit the invoking checkout's automatically exported cache paths.
    $env:CARGO_TARGET_DIR = $TargetDir
    $env:CARGO_BUILD_BUILD_DIR = $BuildDir
}

$identity = if ((Split-Path -Leaf (Split-Path -Parent $repo)) -eq '.worktrees') {
    Split-Path -Leaf $repo
} else { 'main' }
if ($identity -notmatch '^(main|[a-z])$') { throw 'Use a named .worktrees/a through .worktrees/z slot.' }

foreach ($entry in @(
    @{ Link = 'target'; Destination = $(if ($env:CARGO_TARGET_DIR) { $env:CARGO_TARGET_DIR } else { "D:/target/rust_howto/$identity" }) },
    @{ Link = 'build'; Destination = $(if ($env:CARGO_BUILD_BUILD_DIR) { $env:CARGO_BUILD_BUILD_DIR } else { "C:/code_build/rust_howto/$identity" }) },
    @{ Link = '.sccache'; Destination = $(if ($env:SCCACHE_DIR) { $env:SCCACHE_DIR } else { 'D:/sccache' }) }
)) {
    $link = Join-Path $repo $entry.Link
    $destination = [IO.Path]::GetFullPath($entry.Destination)
    $existing = Get-Item -LiteralPath $link -Force -ErrorAction SilentlyContinue
    if ($existing) {
        if ($existing.LinkType -ne 'Junction' -or [IO.Path]::GetFullPath($existing.Target) -ne $destination) {
            throw "Preserving existing $link; expected a junction to $destination."
        }
    } else {
        New-Item -ItemType Directory -Path $destination -Force | Out-Null
        New-Item -ItemType Junction -Path $link -Target $destination | Out-Null
    }
    Write-Host "$link -> $destination"
}
