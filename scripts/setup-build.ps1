# Purpose: Set up this checkout's external Cargo build paths and compiler-cache links.
# Usage: Run with pwsh -File scripts/setup-build.ps1; pass any required parameters.
#
# TODO Reviewed ??-??-2026.

param(
    [ValidatePattern('^[abc]$')][string]$Slot,
    [string]$Branch,
    [switch]$RestartCache,
    [string]$TargetDir,
    [string]$BuildDir
)

$ErrorActionPreference = 'Stop'
$repo = Split-Path -Parent $PSScriptRoot
$commonGitDir = (& git -C $repo rev-parse --path-format=absolute --git-common-dir)
if ($LASTEXITCODE) { throw 'Could not locate the shared Git directory.' }
$mainRepo = Split-Path -Parent $commonGitDir.Trim()
$codeTargetRoot = if ($env:CODE_TARGET_ROOT) { $env:CODE_TARGET_ROOT } else { 'D:\target' }
$codeBuildRoot = if ($env:CODE_BUILD_ROOT) { $env:CODE_BUILD_ROOT } else { 'C:\code_build' }
$codeSccacheRoot = if ($env:CODE_SCCACHE_ROOT) { $env:CODE_SCCACHE_ROOT } elseif ($env:SCCACHE_DIR) { $env:SCCACHE_DIR } else { 'D:\sccache' }
$env:SCCACHE_CACHE_SIZE = '35G'

function Get-CacheBaseDirs {
    $justfile = Join-Path $repo 'justfile'
    $bases = & just --justfile $justfile --evaluate build_sccache_basedirs
    if ($LASTEXITCODE) { throw 'Could not enumerate registered sccache base directories.' }
    return $bases.Trim()
}

if ($RestartCache) {
    if (Get-Process cargo,rustc -ErrorAction SilentlyContinue) {
        throw 'Wait for running Cargo/rustc processes before restarting the shared sccache daemon.'
    }
    # sccache reads BASEDIRS when its daemon starts, not on each client request.
    # Use only this repository's main checkout and registered slots a, b, and c.
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
if ($identity -notmatch '^(main|[abc])$') { throw 'Use only .worktrees/a, .worktrees/b, or .worktrees/c.' }

foreach ($entry in @(
    @{ Link = 'target'; Destination = $(if ($TargetDir) { $TargetDir } else { (Join-Path (Join-Path $codeTargetRoot "rust_howto") $identity) }) },
    @{ Link = 'build'; Destination = $(if ($BuildDir) { $BuildDir } else { (Join-Path (Join-Path $codeBuildRoot "rust_howto") $identity) }) },
    @{ Link = '.sccache'; Destination = $codeSccacheRoot }
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
