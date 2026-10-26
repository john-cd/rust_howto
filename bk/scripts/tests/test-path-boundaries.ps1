$ErrorActionPreference = 'Stop'
Set-StrictMode -Version Latest

$bookRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$repoRoot = Split-Path -Parent $bookRoot
$bookJustfile = Join-Path $bookRoot 'justfile'
$buildJustfile = Join-Path $repoRoot 'scripts/build.just'
$oldExpected = $env:JUST_PATH_BOUNDARY_EXPECTED
$oldBaseDirs = $env:SCCACHE_BASEDIRS

try {
    $paths = @(
        'D:\Rust Howto\folder with spaces\file.txt',
        'D:/Rust Howto/folder with spaces/file.txt'
    )

    foreach ($path in $paths) {
        $env:JUST_PATH_BOUNDARY_EXPECTED = $path
        & just --justfile $bookJustfile _path-boundary-probe $path
        if ($LASTEXITCODE -ne 0) {
            throw "Path argument failed to round-trip: $path (exit $LASTEXITCODE)."
        }
    }

    # The caller's cache base stays byte-for-byte intact; this checkout is
    # appended with forward slashes for sccache's Windows environment value.
    $env:SCCACHE_BASEDIRS = 'E:\Cache Root\Slot'
    $cacheBaseDirs = & just --justfile $buildJustfile --evaluate SCCACHE_BASEDIRS
    if ($LASTEXITCODE -ne 0) {
        throw "Could not evaluate SCCACHE_BASEDIRS (exit $LASTEXITCODE)."
    }
    $entries = $cacheBaseDirs -split ';'
    if ($entries -notcontains 'E:\Cache Root\Slot') {
        throw 'The caller-supplied SCCACHE_BASEDIRS value was changed.'
    }
    $normalizedRepoRoot = $repoRoot.Replace('\', '/')
    if ($entries -notcontains $normalizedRepoRoot) {
        throw "Expected the checkout path in forward-slash form: $normalizedRepoRoot."
    }

    # Confirm a nonzero PowerShell child exit remains a failure in just.
    $env:JUST_PATH_BOUNDARY_EXPECTED = '__boundary_exit_23__'
    & just --justfile $bookJustfile _path-boundary-probe '__boundary_exit_23__'
    if ($LASTEXITCODE -eq 0) {
        throw 'Expected the nonzero child exit status to fail the just recipe.'
    }

    Write-Output 'Windows path boundary checks passed.'
} finally {
    $env:JUST_PATH_BOUNDARY_EXPECTED = $oldExpected
    $env:SCCACHE_BASEDIRS = $oldBaseDirs
}
